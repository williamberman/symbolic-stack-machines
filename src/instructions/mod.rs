mod convert;
use primitive_types::U256;

use crate::{
    machine::{mem_ptr::MemPtr, Machine},
    val::{byte::Byte, word::Word},
};

pub use convert::{parse_bytecode, parse_bytecode_thread_local};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Stop,
    Add,
    Mul,
    Sub,
    Div,
    Lt,
    Gt,
    Slt,
    Eq,
    IsZero,
    And,
    Shr,
    CallValue,
    CallDataLoad,
    CallDataSize,
    Pop,
    MLoad,
    MStore,
    Jump,
    JumpI,
    Jumpdest,
    Push(u8),
    Dup(u8),
    Swap(u8),
    Revert,
    Return,

    // Literal byte, used as data.
    Lit(Byte),

    // Not actual EVM instruction. Used for testing
    Assert(Word),
}

pub type InstructionResult = (Machine, Option<Machine>);

impl Instruction {
    pub fn exec(&self, mut m: Machine) -> InstructionResult {
        let mut cont: Option<Machine> = None;

        match self {
            Instruction::Add => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 + op_2);

                m.pc += 1;
            }
            Instruction::Mul => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 * op_2);

                m.pc += 1;
            }
            Instruction::Sub => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 - op_2);

                m.pc += 1;
            }
            Instruction::Div => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 / op_2);

                m.pc += 1;
            }
            Instruction::Lt => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1._lt(op_2));
                m.pc += 1;
            }
            Instruction::Gt => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1._gt(op_2));
                m.pc += 1;
            }
            Instruction::Slt => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1._slt(op_2));

                m.pc += 1;
            }
            Instruction::Eq => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1._eq_word(op_2));
                m.pc += 1;
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
                    op => op._eq_word(Word::zero()),
                };

                m.stack.push(to_push);

                m.pc += 1;
            }
            Instruction::And => {
                let op_1 = m.stack.pop().unwrap();
                let op_2 = m.stack.pop().unwrap();

                m.stack.push(op_1 & op_2);

                m.pc += 1;
            }
            Instruction::Shr => {
                let shift = m.stack.pop().unwrap();
                let value = m.stack.pop().unwrap();
                m.stack.push(value >> shift);
                m.pc += 1;
            }
            Instruction::Push(n) => {
                let n_bytes = *n as usize;
                let val = Word::from_bytes_vec(&m.pgm, m.pc + 1, n_bytes, false);
                m.stack.push(val);
                m.pc += n_bytes + 1;
            }
            Instruction::Dup(n) => {
                let val = m.stack.peek_n(*n as usize - 1).unwrap().clone();
                m.stack.push(val);
                m.pc += 1;
            }
            Instruction::Swap(n) => {
                let as_usize = *n as usize;
                let top = m.stack.peek().unwrap().clone();
                let nth = m.stack.peek_n(as_usize).unwrap().clone();
                m.stack.set(0, nth);
                m.stack.set(as_usize, top);
                m.pc += 1;
            }
            Instruction::Stop => {
                m.halt = true;
            }
            Instruction::Jump => {
                let dest: U256 = m.stack.pop().unwrap().into();
                m.pc = dest.as_usize();
            }
            Instruction::JumpI => {
                let dest: U256 = m.stack.pop().unwrap().into();
                let cond = m.stack.pop().unwrap();

                match cond {
                    Word::C(cond) => {
                        if cond != U256::zero() {
                            m.pc = dest.as_usize();
                        } else {
                            m.pc += 1;
                        }
                    }
                    cond => {
                        let mut falls_through = m.clone();
                        // m takes target

                        let falls_through_cond = cond._eq(Word::zero());
                        let takes_target_cond = !falls_through_cond.clone();

                        falls_through.constraints.push_back(falls_through_cond);
                        m.constraints.push_back(takes_target_cond);

                        falls_through.pc += 1;
                        m.pc = dest.as_usize();

                        cont = Some(falls_through);
                    }
                }
            }
            Instruction::Jumpdest => {
                m.pc += 1;
            }
            Instruction::MLoad => {
                let mem_idx = m.stack.pop().unwrap();
                let mem_val = m.mem.read_word(mem_idx.clone());

                m.stack.push(mem_val);

                m.pc += 1;
            }
            Instruction::MStore => {
                let offset = m.stack.pop().unwrap();
                let val = m.stack.pop().unwrap();

                m.mem.write_word(offset, val);

                m.pc += 1;
            }
            Instruction::CallValue => {
                m.stack.push(m.call_value.clone());
                m.pc += 1;
            }
            Instruction::CallDataLoad => {
                let idx = m.stack.pop().unwrap();
                let val = m.calldata.read_word(idx);
                m.stack.push(val);
                m.pc += 1;
            }
            Instruction::CallDataSize => {
                m.stack.push(m.calldata.size());
                m.pc += 1;
            }
            Instruction::Pop => {
                m.stack.pop();
                m.pc += 1;
            }
            Instruction::Return => {
                let offset = m.stack.pop().unwrap();
                let length = m.stack.pop().unwrap();
                m.return_ptr = Some(MemPtr { offset, length });
                m.halt = true;
            }
            Instruction::Revert => {
                let offset = m.stack.pop().unwrap();
                let length = m.stack.pop().unwrap();
                m.revert_ptr = Some(MemPtr { offset, length });
                m.halt = true;
            }
            Instruction::Lit(x) => {
                panic!("literal instruction {:?}", x);
            }
            Instruction::Assert(w) => {
                let op = m.stack.peek().unwrap().clone();

                m.constraints.push_back(op._eq(w.clone()));

                m.pc += 1;
            }
        }

        (m, cont)
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

pub fn jump() -> Instruction {
    Instruction::Jump
}

pub fn jumpi() -> Instruction {
    Instruction::JumpI
}

pub fn stop() -> Instruction {
    Instruction::Stop
}

pub fn mstore() -> Instruction {
    Instruction::MStore
}

pub fn calldataload() -> Instruction {
    Instruction::CallDataLoad
}

pub fn shr() -> Instruction {
    Instruction::Shr
}

pub fn assert_ins<T: Into<Word>>(x: T) -> Instruction {
    Instruction::Assert(x.into())
}
