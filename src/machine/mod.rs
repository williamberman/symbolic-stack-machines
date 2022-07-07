use std::rc::Rc;

use im::Vector;

use crate::{
    calldata::Calldata, environment::Env, instructions::Instruction, memory::Memory, stack::Stack,
    val::constraint::Constraint, z3::solve_z3,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Machine {
    pub stack: Stack,
    pub mem: Memory,
    pub env: Env,
    pub pc: usize,
    pub pgm: Rc<Vec<Instruction>>,
    pub calldata: Rc<Calldata>,
    pub constraints: Vector<Constraint>,
    pub halt: bool,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            stack: Default::default(),
            mem: Default::default(),
            env: Default::default(),
            pc: 0,
            pgm: Default::default(),
            calldata: Default::default(),
            constraints: Default::default(),
            halt: false,
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
                                Some(_) => queue.push(m),
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
}
