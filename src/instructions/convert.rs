use std::thread::LocalKey;

use crate::val::byte::Byte;

use super::Instruction;

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::Stop => 0x00,
            Instruction::Add => 0x01,
            Instruction::Mul => 0x02,
            Instruction::Sub => 0x03,
            Instruction::Div => 0x04,
            Instruction::Lt => 0x10,
            Instruction::Gt => 0x11,
            Instruction::Slt => 0x12,
            Instruction::Eq => 0x14,
            Instruction::IsZero => 0x15,
            Instruction::And => 0x16,
            Instruction::Shr => 0x1C,
            Instruction::CallValue => 0x34,
            Instruction::CallDataLoad => 0x35,
            Instruction::CallDataSize => 0x36,
            Instruction::Pop => 0x50,
            Instruction::MLoad => 0x51,
            Instruction::MStore => 0x52,
            Instruction::Jump => 0x56,
            Instruction::JumpI => 0x57,
            Instruction::Jumpdest => 0x5B,
            Instruction::Push(n) => 0x60 + n - 1,
            Instruction::Dup(n) => 0x80 + n - 1,
            Instruction::Swap(n) => 0x90 + n - 1,
            Instruction::Return => 0xF3,
            Instruction::Revert => 0xFD,
            Instruction::Lit(x) => {
                match x {
                    Byte::C(x, _) => x,
                    // 0xfe is invalid opcode
                    _ => 0xfe,
                }
            }
            Instruction::Assert(_) => 0xfe,
        }
    }
}

impl Into<Byte> for Instruction {
    fn into(self) -> Byte {
        match self {
            Instruction::Lit(x) => x,
            x => Byte::C(x.into(), None),
        }
    }
}

impl From<u8> for Instruction {
    fn from(x: u8) -> Self {
        if x >= 0x60 && x <= 0x7f {
            return Instruction::Push(x - 0x5F);
        }

        if x >= 0x80 && x <= 0x8f {
            return Instruction::Dup(x - 0x7F);
        }

        if x >= 0x90 && x <= 0x9F {
            return Instruction::Swap(x - 0x8F);
        }

        match x {
            0x00 => Instruction::Stop,
            0x01 => Instruction::Add,
            0x02 => Instruction::Mul,
            0x03 => Instruction::Sub,
            0x04 => Instruction::Div,
            0x10 => Instruction::Lt,
            0x11 => Instruction::Gt,
            0x12 => Instruction::Slt,
            0x14 => Instruction::Eq,
            0x15 => Instruction::IsZero,
            0x16 => Instruction::And,
            0x1C => Instruction::Shr,
            0x34 => Instruction::CallValue,
            0x35 => Instruction::CallDataLoad,
            0x36 => Instruction::CallDataSize,
            0x50 => Instruction::Pop,
            0x51 => Instruction::MLoad,
            0x52 => Instruction::MStore,
            0x56 => Instruction::Jump,
            0x57 => Instruction::JumpI,
            0x5B => Instruction::Jumpdest,
            0xF3 => Instruction::Return,
            0xFD => Instruction::Revert,
            x => Instruction::Lit(x.into()),
        }
    }
}

pub fn parse_bytecode(bytecode: &str) -> Vec<Instruction> {
    hex::decode(bytecode)
        .unwrap()
        .into_iter()
        .map(|x| Instruction::from(x))
        .collect()
}

pub fn parse_bytecode_thread_local(bytecode: &'static LocalKey<String>) -> Vec<Instruction> {
    bytecode.with(|x| parse_bytecode(x.as_str()))
}
