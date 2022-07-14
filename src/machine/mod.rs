pub mod mem_ptr;
mod sym_results;

use std::time::Instant;
use std::{
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use im::Vector;
use log::info;
use z3::SatResult;

use crate::z3::make_solve_results;
use crate::{
    calldata::Calldata,
    instructions::{Instruction, InstructionResult},
    memory::Memory,
    stack::Stack,
    val::{byte::Byte, constraint::Constraint, word::Word},
    z3::{make_z3_config, make_z3_constraint, SolveResults},
};

use self::{
    mem_ptr::MemPtr,
    sym_results::{SymResults, SymResultsWithSolver},
};

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
        let mut rv = SymResultsWithSolver::new(self);

        loop {
            let start_branch = rv.queue.pop();
            if let Some(mach) = start_branch {
                if !mach.halt {
                    let n_constraints = mach.constraints.len();
                    let (new_machine, branch) = mach.step_sym();

                    let mut new_machines = vec![new_machine];

                    if let Some(branch) = branch {
                        new_machines.push(branch);
                    }

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

            info!(
                "queue: {}, leaves: {}, pruned: {}",
                rv.queue.len(),
                rv.leaves.len(),
                rv.pruned.len()
            );
        }

        rv.into()
    }

    pub fn run_sym_inc(self) -> SymResults {
        let cfg = make_z3_config();
        let ctx = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&ctx);

        let mut cur: Option<Machine> = Some(self);
        let mut work_stack: Vec<Machine> = vec![];

        let mut pruned: Vec<Machine> = vec![];
        let mut leaves: Vec<Machine> = vec![];

        loop {
            match cur {
                Some(m) => {
                    if m.halt {
                        leaves.push(m);
                        cur = None;
                        solver.pop(1);
                    } else {
                        let (new_machine, branch) = m.step_sym();

                        match branch {
                            None => {
                                // No new constraints added
                                cur = Some(new_machine);
                            }

                            // New constraints added, on next loop try to take branches
                            Some(branch) => {
                                cur = None;
                                work_stack.push(new_machine);
                                work_stack.push(branch);
                            }
                        }
                    }
                }
                None => {
                    match work_stack.pop() {
                        Some(mut m) => {
                            solver.push();

                            let z3_constraint = make_z3_constraint(
                                &ctx,
                                m.constraints.get(m.constraints.len() - 1).unwrap(),
                            );

                            solver.assert(&z3_constraint);

                            let timer = Instant::now();

                            info!("solving num_constaints: {}", m.constraints.len(),);

                            let solver_res = solver.check();

                            let elapsed = timer.elapsed();

                            info!("time elapsed: {:.2?}, result: {:?}", elapsed, solver_res);

                            if solver_res != SatResult::Sat {
                                solver.pop(1);
                                pruned.push(m);
                            } else {
                                let model = solver.get_model().unwrap();
                                let solve_results = make_solve_results(
                                    &ctx,
                                    model,
                                    vec![],
                                    m.calldata.inner().clone(),
                                );
                                m.solve_results = Some(solve_results);
                                cur = Some(m);
                            }
                        }

                        // There is no current machine to step and the work stack is empty.
                        // Exit the loop
                        None => break,
                    }
                }
            }

            let work_stack_len_add = match cur {
                Some(_) => 1,
                None => 0,
            };

            info!(
                "work_stack: {}, leaves: {}, pruned: {}",
                work_stack.len() + work_stack_len_add,
                leaves.len(),
                pruned.len()
            );
        }

        SymResults { leaves, pruned }
    }

    pub fn step(self) -> Machine {
        let i = self.pgm.get(self.pc).unwrap().clone();

        // Assume only one is returned
        i.exec(self).0
    }

    pub fn step_sym(self) -> InstructionResult {
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
