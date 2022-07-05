use crate::{machine::Machine, val::word::Word};

#[derive(Clone)]
pub struct Constraint {}

#[derive(Clone, Debug)]
pub enum Instruction {
    Add,
    Sub,
    IsZero,
    Push(u8),
    Stop,
    JumpI,
    MLoad,
    MStore,
    Lit(u8),
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::Stop => 0x00,
            Instruction::Add => 0x01,
            Instruction::Sub => 0x03,
            Instruction::IsZero => 0x15,
            Instruction::MLoad => 0x51,
            Instruction::MStore => 0x52,
            Instruction::JumpI => 0x57,
            Instruction::Push(n) => 0x60 + n - 1,
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
            Instruction::Push(n) => {
                let n_bytes = *n as usize;
                let val = Word::from_bytes_vec(&m.pgm, m.pc.unwrap() + 1, n_bytes);
                m.stack.push(val);
                m.pc = m.pc.map(|x| x + n_bytes + 1);
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

    pub fn as_bytes(pgm: Vec<Instruction>) -> Vec<u8> {
        pgm.into_iter().map(|x| { x.into() }).collect()
    }
}

pub fn push1() -> Instruction {
    Instruction::Push(1)
}

pub fn add() -> Instruction {
    Instruction::Add
}

pub fn sub() -> Instruction {
    Instruction::Sub
}

pub fn lit(b: u8) -> Instruction {
    Instruction::Lit(b)
}

pub fn lit_32<T>(val: T) -> [Instruction; 32] where Word: From<T> {
    Word::constant_instruction(val)
}
