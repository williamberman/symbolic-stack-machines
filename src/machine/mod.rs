pub mod mem_ptr;
use std::rc::Rc;

use im::Vector;

use crate::{
    calldata::Calldata,
    instructions::Instruction,
    memory::Memory,
    stack::Stack,
    val::{byte::Byte, constraint::Constraint, word::Word},
    z3::solve_z3,
};

use self::mem_ptr::MemPtr;

#[derive(Debug, PartialEq, Eq)]
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
}

impl Clone for Machine {
    fn clone(&self) -> Self {
        Self {
            id: self.id + 1,
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
        }
    }
}

#[derive(Debug)]
pub struct SymResults {
    pub leaves: Vec<Machine>,
    pub pruned: Vec<Machine>,
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
        let mut queue: Vec<Machine> = vec![self];

        let mut leaves: Vec<Machine> = vec![];
        let mut pruned: Vec<Machine> = vec![];

        loop {
            let start_branch = queue.pop();
            if let Some(mach) = start_branch {
                if !mach.halt {
                    let new_machines = mach.step_sym();
                    new_machines.into_iter().for_each(|m| {
                        if m.constraints.is_empty() {
                            queue.push(m)
                        } else {
                            match solve_z3(&m.constraints, vec![], vec![]) {
                                Some(_) => {
                                    if m.halt {
                                        leaves.push(m)
                                    } else {
                                        queue.push(m)
                                    }
                                },
                                None => pruned.push(m),
                            }
                        }
                    });
                } else {
                    leaves.push(mach);
                }
            } else {
                break;
            }
        }

        SymResults { leaves, pruned }
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
        self.revert_ptr
            .clone()
            .map(|ptr| self.mem_ptr_bytes(ptr))
    }

    pub fn revert_string(&self) -> Option<String> {
        self.revert_ptr
            .clone()
            .map(|ptr| self.mem_ptr_string(ptr))
    }

    pub fn return_bytes(&self) -> Option<Vec<Byte>> {
        self.return_ptr
            .clone()
            .map(|ptr| self.mem_ptr_bytes(ptr))
    }

    pub fn return_string(&self) -> Option<String> {
        self.return_ptr
            .clone()
            .map(|ptr| self.mem_ptr_string(ptr))
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
