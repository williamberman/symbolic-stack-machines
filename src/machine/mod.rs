use std::rc::Rc;

use im::Vector;

use crate::{
    environment::Env, instructions::Instruction, memory::Memory, stack::Stack,
    val::constraint::Constraint, z3::solve_z3,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Machine {
    pub stack: Stack,
    pub mem: Memory,
    pub env: Env,
    pub pc: Option<usize>,
    pub pgm: Rc<Vec<Instruction>>,
    pub constraints: Vector<Constraint>,
}

#[derive(Debug)]
pub struct SymResults {
    pub leaves: Vec<Machine>,
    pub pruned: Vec<Machine>,
}

impl Machine {
    pub fn new(pgm: Vec<Instruction>) -> Self {
        let env = Env {};
        let pc = Some(0);
        let mem = Memory::default();
        let stack = Stack::default();

        Machine {
            stack,
            mem,
            env,
            pc,
            pgm: Rc::new(pgm),
            constraints: Vector::new(),
        }
    }

    pub fn run(self) -> Machine {
        let mut x = self;

        while x.can_continue() {
            x = x.step();
        }

        x
    }

    pub fn run_sym(self) -> SymResults {
        let mut trace_tree: Vec<Machine> = vec![self];

        let mut leaves: Vec<Machine> = vec![];
        let mut pruned: Vec<Machine> = vec![];

        loop {
            let start_branch = trace_tree.pop();
            if let Some(mach) = start_branch {
                if mach.can_continue() {
                    let new_machines = mach.step_sym();
                    new_machines.into_iter().for_each(|m| {
                        if m.constraints.is_empty() {
                            trace_tree.push(m)
                        } else {
                            match solve_z3(&m.constraints, vec![], vec![]) {
                                Some(_) => trace_tree.push(m),
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
        let i = self.pgm.get(self.pc.unwrap()).unwrap().clone();

        // Assume only one is returned
        i.exec(self).pop().unwrap()
    }

    pub fn step_sym(self) -> Vec<Self> {
        let pc = self.pc.unwrap();

        let i = self.pgm.get(pc).unwrap().clone();

        i.exec(self)
    }

    pub fn can_continue(&self) -> bool {
        match self.pc {
            Some(pc) => pc < self.pgm.len(),
            None => false,
        }
    }
}
