pub mod mem_ptr;
use std::{
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use im::Vector;

use crate::{
    calldata::Calldata,
    instructions::Instruction,
    memory::Memory,
    stack::Stack,
    val::{byte::Byte, constraint::Constraint, word::Word},
    z3::{solve_z3, SolveResults},
};

use self::mem_ptr::MemPtr;

#[derive(Debug)]
pub struct Machine {
    pub id: usize,
    pub stack: Stack,
    pub mem: Memory,
    pub pc: usize,
    pub pgm: Rc<Vec<Instruction>>,
    pub calldata: Rc<Calldata>,
    pub constraints: Vector<Constraint>,
    pub halt: bool,
    pub call_value: Word,
    pub return_ptr: Option<MemPtr>,
    pub revert_ptr: Option<MemPtr>,

    pub constraint_solve: bool,
    pub ctr: Arc<AtomicUsize>,
    pub solve_results: Option<SolveResults>,
}

impl Clone for Machine {
    fn clone(&self) -> Self {
        let new_id = self.ctr.fetch_add(1, Ordering::SeqCst);

        Self {
            id: new_id,
            stack: self.stack.clone(),
            mem: self.mem.clone(),
            pc: self.pc.clone(),
            pgm: self.pgm.clone(),
            calldata: self.calldata.clone(),
            constraints: self.constraints.clone(),
            halt: self.halt.clone(),
            call_value: self.call_value.clone(),
            return_ptr: self.return_ptr.clone(),
            revert_ptr: self.revert_ptr.clone(),

            constraint_solve: self.constraint_solve,
            ctr: self.ctr.clone(),
            solve_results: self.solve_results.clone(),
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            id: Default::default(),
            stack: Default::default(),
            mem: Default::default(),
            pc: 0,
            pgm: Default::default(),
            calldata: Default::default(),
            constraints: Default::default(),
            halt: false,
            call_value: Default::default(),
            return_ptr: Default::default(),
            revert_ptr: Default::default(),

            constraint_solve: true,
            ctr: Arc::new(AtomicUsize::new(1)),
            solve_results: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct SymResults {
    queue: Vec<Machine>,
    pub leaves: Vec<Machine>,
    pub pruned: Vec<Machine>,
}

impl SymResults {
    fn new(m: Machine) -> Self {
        SymResults {
            queue: vec![m],
            leaves: vec![],
            pruned: vec![],
        }
    }

    fn push(&mut self, mut m: Machine, constraint_solve: bool) {
        if constraint_solve && !m.constraints.is_empty() {
            match solve_z3(&m.constraints, vec![], m.calldata.inner().clone()) {
                Some(sr) => {
                    m.solve_results = Some(sr);
                    self.push_inner(m);
                }
                None => self.pruned.push(m),
            }
        } else {
            self.push_inner(m)
        }
    }

    fn push_inner(&mut self, m: Machine) {
        if m.halt {
            self.leaves.push(m)
        } else {
            self.queue.push(m)
        }
    }
}

impl Machine {
    pub fn new(pgm: Vec<Instruction>) -> Self {
        let mut m = Self::default();
        m.pgm = Rc::new(pgm);
        m
    }

    pub fn run(self) -> Machine {
        let mut x = self;

        while !x.halt {
            x = x.step();
        }

        x
    }

    pub fn run_sym(self) -> SymResults {
        let mut rv = SymResults::new(self);

        loop {
            let start_branch = rv.queue.pop();
            if let Some(mach) = start_branch {
                if !mach.halt {
                    let n_constraints = mach.constraints.len();
                    let new_machines = mach.step_sym();

                    new_machines.into_iter().for_each(|m| {
                        // Do not constraint solve when number constraints doesn't change
                        // because constraints can only be added
                        let cs = m.constraint_solve && m.constraints.len() != n_constraints;
                        rv.push(m, cs);
                    });
                } else {
                    rv.leaves.push(mach);
                }
            } else {
                break;
            }
        }

        rv
    }

    pub fn step(self) -> Machine {
        let i = self.pgm.get(self.pc).unwrap().clone();

        // Assume only one is returned
        i.exec(self).pop().unwrap()
    }

    pub fn step_sym(self) -> Vec<Self> {
        let i = self.pgm.get(self.pc).unwrap().clone();

        i.exec(self)
    }

    pub fn revert_bytes(&self) -> Option<Vec<Byte>> {
        self.revert_ptr.clone().map(|ptr| self.mem_ptr_bytes(ptr))
    }

    pub fn revert_string(&self) -> Option<String> {
        self.revert_ptr.clone().map(|ptr| self.mem_ptr_string(ptr))
    }

    pub fn return_bytes(&self) -> Option<Vec<Byte>> {
        self.return_ptr.clone().map(|ptr| self.mem_ptr_bytes(ptr))
    }

    pub fn return_string(&self) -> Option<String> {
        self.return_ptr.clone().map(|ptr| self.mem_ptr_string(ptr))
    }

    fn mem_ptr_bytes(&self, ptr: MemPtr) -> Vec<Byte> {
        self.mem.read_bytes(ptr.offset, ptr.length)
    }

    fn mem_ptr_string(&self, ptr: MemPtr) -> String {
        let bytes = self.mem_ptr_bytes(ptr);
        let bs: Vec<u8> = bytes.into_iter().map(|x| x.into()).collect();
        hex::encode(bs)
    }
}
