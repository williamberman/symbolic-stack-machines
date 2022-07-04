use crate::environment::EnvRecord;
use crate::machine::Machine;
use crate::memory::*;
use crate::stack::*;

#[derive(Clone)]
pub struct Constraint {}

pub struct ExecRecord {
    pub stack_diff: Option<StackRecord>,
    pub mem_diff: Option<MemRecord>,
    pub env_diff: Option<EnvRecord>,
    pub pc_change: Option<usize>,
    pub halt: bool,
    pub constraints: Option<Vec<Constraint>>,
}

impl Default for ExecRecord {
    fn default() -> Self {
        Self {
            stack_diff: None,
            mem_diff: None,
            env_diff: None,
            pc_change: None,
            halt: false,
            constraints: None,
        }
    }
}

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
    pub fn exec(&self, m: &Machine) -> Vec<ExecRecord> {
        match self {
            Instruction::Add => {
                let mut change_log = ExecRecord::default();

                let op_1 = m.stack.peek(0).unwrap();
                let op_2 = m.stack.peek(1).unwrap();
                let res = op_1.clone() + op_2.clone();

                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop,
                        StackOpRecord::Pop,
                        StackOpRecord::Push(res),
                    ],
                });

                vec![change_log]
            }
            Instruction::Sub => {
                let mut change_log = ExecRecord::default();

                let op_1 = m.stack.peek(0).unwrap();
                let op_2 = m.stack.peek(1).unwrap();
                let res = op_1.clone() - op_2.clone();

                change_log.stack_diff = Some(StackRecord {
                    changed: vec![
                        StackOpRecord::Pop,
                        StackOpRecord::Pop,
                        StackOpRecord::Push(res),
                    ],
                });

                vec![change_log]
            }
            Instruction::IsZero => {
                let mut change_log = ExecRecord::default();

                let op = m.stack.peek(0).unwrap();

                let rv = op._eq(&ZERO).ite(ONE, ZERO);

                change_log.stack_diff = Some(StackRecord {
                    changed: vec![StackOpRecord::Pop, StackOpRecord::Push(rv)],
                });

                vec![change_log]
            }
            Instruction::Push(x) => {
                let mut change_log = ExecRecord::default();

                change_log.stack_diff = Some(StackRecord {
                    changed: vec![StackOpRecord::Push(x.clone())],
                });

                vec![change_log]
            }
            Instruction::Stop => {
                let mut change_log = ExecRecord::default();
                change_log.halt = true;
                vec![change_log]
            }
            Instruction::JumpI => {
                let mut change_log = ExecRecord::default();

                let dest = m.stack.peek(0).unwrap();
                let cond = m.stack.peek(1).unwrap();

                if *cond != ZERO {
                    let x = Into::<usize>::into(*dest);
                    change_log.pc_change = Some(x);
                }

                vec![change_log]
            }
            Instruction::MLoad => {
                let mut change_log = ExecRecord::default();

                let mem_idx = m.stack.peek(0).unwrap();
                let mem_val = m.mem.read_word(mem_idx.clone()).unwrap();

                change_log.stack_diff = Some(StackRecord {
                    changed: vec![StackOpRecord::Pop, StackOpRecord::Push(mem_val)],
                });

                vec![change_log]
            }
            Instruction::MStore => {
                let mut change_log = ExecRecord::default();

                let mem_idx = m.stack.peek(0).unwrap();
                let mem_val = m.stack.peek(1).unwrap();

                change_log.stack_diff = Some(StackRecord {
                    changed: vec![StackOpRecord::Pop, StackOpRecord::Pop],
                });

                change_log.mem_diff = Some(MemRecord {
                    changed: vec![MemOpRecord::Write(*mem_idx, *mem_val)],
                });

                vec![change_log]
            }
        }
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