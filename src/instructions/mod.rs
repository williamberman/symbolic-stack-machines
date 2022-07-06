use primitive_types::U256;

use crate::{
    machine::Machine,
    val::{byte::Byte, word::Word},
};

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
    Lit(Byte),
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
            Instruction::Lit(x) => {
                match x {
                    Byte::C(x) => x,
                    // 0xfe is invalid opcode
                    _ => 0xfe,
                }
            }
        }
    }
}

impl Into<Byte> for Instruction {
    fn into(self) -> Byte {
        match self {
            Instruction::Lit(x) => x,
            x => Byte::C(x.into()),
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

                let to_push = match op {
                    Word::C(op) => {
                        if op == U256::zero() {
                            Word::one()
                        } else {
                            Word::zero()
                        }
                    }
                    op => {
                        op._eq(Word::zero()).ite(Word::one(), Word::zero())
                    }
                };

                m.stack.push(to_push);
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
                let dest: U256 = m.stack.pop().unwrap().into();
                let cond = m.stack.pop().unwrap();

                match cond {
                    Word::C(cond) => {
                        if cond != U256::zero() {
                            m.pc = Some(dest.as_usize());
                        } else {
                            m.pc = m.pc.map(|x| x + 1);
                        }

                        cont.push(m);
                    }
                    cond => {
                        let mut falls_through = m.clone();
                        let mut takes_target = m;

                        let falls_through_cond = cond._eq(Word::zero());
                        let takes_target_cond = !falls_through_cond.clone();

                        falls_through.constraints.push_back(falls_through_cond);
                        takes_target.constraints.push_back(takes_target_cond);

                        falls_through.pc = falls_through.pc.map(|x| x + 1);
                        takes_target.pc = Some(dest.as_usize());

                        cont.push(falls_through);
                        cont.push(takes_target);
                    }
                }
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
                panic!("literal instruction {:?}", x);
            }
        }

        cont
    }

    pub fn as_bytes(pgm: Vec<Instruction>) -> Vec<u8> {
        pgm.into_iter().map(|x| x.into()).collect()
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

pub fn lit<T: Into<Byte>>(x: T) -> Instruction {
    Instruction::Lit(x.into())
}

pub fn lit_32<T>(val: T) -> [Instruction; 32]
where
    Word: From<T>,
{
    Word::constant_instruction(val)
}

pub fn iszero() -> Instruction {
    Instruction::IsZero
}

pub fn jumpi() -> Instruction {
    Instruction::JumpI
}

pub fn stop() -> Instruction {
    Instruction::Stop
}
