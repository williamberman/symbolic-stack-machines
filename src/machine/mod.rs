use crate::{
    environment::Env,
    instructions::{Constraint, ExecRecord, Instruction},
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
        let i = self.pgm.get(self.pc.unwrap()).unwrap();

        // Assume only one is returned
        let exec_record = i.exec(&self).pop().unwrap();

        self.apply(exec_record)
    }

    pub fn step_sym(self) -> Vec<Self> {
        let pc = self.pc.unwrap();

        let i = self.pgm.get(pc).unwrap();

        let exec_records = i.exec(&self);

        exec_records
            .into_iter()
            .map(|exec_record| self.clone().apply(exec_record))
            .collect()
    }

    pub fn apply(self, r: ExecRecord) -> Self {
        let mut stack = self.stack;
        let mut mem = self.mem;
        let mut env = self.env;
        let mut constraints = self.constraints;

        stack = {
            if let Some(stack_diff) = r.stack_diff {
                stack.apply(stack_diff)
            } else {
                stack
            }
        };

        mem = {
            if let Some(mem_diff) = r.mem_diff {
                mem.apply(mem_diff)
            } else {
                mem
            }
        };

        env = {
            if let Some(env_diff) = r.env_diff {
                env.apply(env_diff)
            } else {
                env
            }
        };

        if let Some(constraints_diff) = r.constraints {
            constraints.extend(constraints_diff);
        }

        let pc = if r.halt {
            None
        } else {
            match r.pc_change {
                Some(pc_change) => Some(pc_change),
                None => Some(self.pc.unwrap() + 1),
            }
        };

        Machine {
            stack,
            mem,
            env,
            pc,
            pgm: self.pgm,
            constraints,
        }
    }

    pub fn can_continue(&self) -> bool {
        match self.pc {
            Some(pc) => pc < self.pgm.len(),
            None => false,
        }
    }
}
