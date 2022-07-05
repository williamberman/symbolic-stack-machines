use crate::{
    environment::Env,
    instructions::{Constraint, Instruction},
    memory::Memory,
    stack::Stack,
};

#[derive(Clone)]
pub struct Machine {
    pub stack: Stack,
    pub mem: Memory,
    pub env: Env,
    pub pc: Option<usize>,
    pub pgm: Vec<Instruction>,
    pub constraints: Vec<Constraint>,
}

impl Machine {
    pub fn run(self) -> Machine {
        let mut x = self;

        while x.can_continue() {
            x = x.step();
        }

        x
    }

    pub fn run_sym(self) -> Vec<Self> {
        let mut trace_tree: Vec<Machine> = vec![self];

        let mut leaves: Vec<Machine> = vec![];

        loop {
            let start_branch = trace_tree.pop();
            if let Some(mach) = start_branch {
                if mach.can_continue() {
                    let new_machines = mach.step_sym();
                    trace_tree.extend(new_machines);
                } else {
                    leaves.push(mach);
                }
            } else {
                break;
            }
        }

        leaves
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
