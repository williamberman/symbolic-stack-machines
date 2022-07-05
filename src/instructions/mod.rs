use crate::{machine::Machine, val::word::Word};

#[derive(Clone)]
pub struct Constraint {}

#[derive(Clone)]
pub enum Instruction {
    Add,
    Sub,
    IsZero,
    Push,
    Stop,
    JumpI,
    MLoad,
    MStore,
    Lit(u8),
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::Add => todo!(),
            Instruction::Sub => todo!(),
            Instruction::IsZero => todo!(),
            Instruction::Push => todo!(),
            Instruction::Stop => todo!(),
            Instruction::JumpI => todo!(),
            Instruction::MLoad => todo!(),
            Instruction::MStore => todo!(),
            Instruction::Lit(x) => x,
        }
    }
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

                m.stack.push(op._eq(&Word::zero()).ite(Word::one(), Word::zero()));

                m.pc = m.pc.map(|x| x + 1);

                cont.push(m);
            }
            Instruction::Push => {
                let val = Word::from_bytes_vec(&m.pgm, m.pc.unwrap());
                m.stack.push(val);
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

                if cond != Word::zero() {
                    let x = Into::<usize>::into(dest);
                    m.pc = Some(x);
                } else {
                    m.pc = m.pc.map(|x| x + 1);
                }

                cont.push(m);
            }
            Instruction::MLoad => {
                let mem_idx = m.stack.pop().unwrap();
                let mem_val = m.mem.read_word(mem_idx.clone());

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
            Instruction::Lit(x) => {
                panic!("literal instruction {}", x);
            }
        }

        cont
    }
}

pub fn push() -> Instruction {
    Instruction::Push
}

pub fn add() -> Instruction {
    Instruction::Add
}

pub fn sub() -> Instruction {
    Instruction::Sub
}
