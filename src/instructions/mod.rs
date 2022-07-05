use crate::{machine::Machine, val::word::{ZERO_WORD, Word, ONE_WORD}};

#[derive(Clone)]
pub struct Constraint {}

#[derive(Clone)]
pub enum Instruction {
    Add,
    Sub,
    IsZero,
    Push(Word),
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

                m.pc = m.pc.map(|x| x + 1);

                cont.push(m);
            }
            Instruction::Sub => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 - op_2);

                m.pc = m.pc.map(|x| x + 1);

                cont.push(m);
            }
            Instruction::IsZero => {
                let op = m.stack.pop().unwrap();

                m.stack.push(op._eq(&ZERO_WORD).ite(ONE_WORD, ZERO_WORD));

                m.pc = m.pc.map(|x| x + 1);

                cont.push(m);
            }
            Instruction::Push(x) => {
                m.stack.push(x.clone());
                m.pc = m.pc.map(|x| x + 1);
                cont.push(m);
            }
            Instruction::Stop => {
                m.pc = None;
                cont.push(m);
            }
            Instruction::JumpI => {
                let dest = m.stack.pop().unwrap();
                let cond = m.stack.pop().unwrap();

                if cond != ZERO_WORD {
                    let x = Into::<usize>::into(dest);
                    m.pc = Some(x);
                } else {
                    m.pc = m.pc.map(|x| x + 1);
                }

                cont.push(m);
            }
            Instruction::MLoad => {
                let mem_idx = m.stack.pop().unwrap();
                let mem_val = m.mem.read_word(mem_idx.clone()).unwrap();

                m.stack.push(mem_val);

                m.pc = m.pc.map(|x| x + 1);

                cont.push(m);
            }
            Instruction::MStore => {
                let idx = m.stack.pop().unwrap();
                let val = m.stack.pop().unwrap();

                m.mem.write_word(idx, val);

                m.pc = m.pc.map(|x| x + 1);

                cont.push(m);
            }
        }

        cont
    }
}

pub fn push<T: Into<Word>>(x: T) -> Instruction {
    Instruction::Push(x.into())
}

pub fn add() -> Instruction {
    Instruction::Add
}

pub fn sub() -> Instruction {
    Instruction::Sub
}
