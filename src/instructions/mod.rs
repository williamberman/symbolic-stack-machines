use crate::{machine::Machine, stack::{ZERO, ONE, StackVal}};

#[derive(Clone)]
pub struct Constraint {}

#[derive(Clone)]
pub enum Instruction {
    Add,
    Sub,
    IsZero,
    Push(StackVal),
    Stop,
    JumpI,
    MLoad,
    MStore,
}

impl Instruction {
    pub fn exec(&self, mut m: Machine) -> Vec<Machine> {
        let mut cont = vec![];

        match self {
            Instruction::Add => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 + op_2);

                cont.push(m);
            }
            Instruction::Sub => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 - op_2);

                cont.push(m);
            }
            Instruction::IsZero => {
                let op = m.stack.pop().unwrap();

                m.stack.push(op._eq(&ZERO).ite(ONE, ZERO));

                cont.push(m);
            }
            Instruction::Push(x) => {
                m.stack.push(x.clone());

                cont.push(m);
            }
            Instruction::Stop => {
                m.pc = None;

                cont.push(m);
            }
            Instruction::JumpI => {
                let dest = m.stack.pop().unwrap();
                let cond = m.stack.pop().unwrap();

                if cond != ZERO {
                    let x = Into::<usize>::into(dest);
                    m.pc = Some(x);
                }

                cont.push(m);
            }
            Instruction::MLoad => {
                let mem_idx = m.stack.pop().unwrap();
                let mem_val = m.mem.read_word(mem_idx.clone()).unwrap();

                m.stack.push(mem_val);

                cont.push(m);
            }
            Instruction::MStore => {
                let idx = m.stack.pop().unwrap();
                let val = m.stack.pop().unwrap();

                m.mem.write_word(idx, val);

                cont.push(m);
            }
        }

        cont
    }
}

pub fn push<T: Into<StackVal>>(x: T) -> Instruction {
    Instruction::Push(x.into())
}

pub fn add() -> Instruction {
    Instruction::Add
}

pub fn sub() -> Instruction {
    Instruction::Sub
}
