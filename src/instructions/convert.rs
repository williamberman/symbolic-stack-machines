use crate::val::byte::Byte;

use super::Instruction;

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        match self {
            Instruction::Stop => 0x00,
            Instruction::Add => 0x01,
            Instruction::Sub => 0x03,
            Instruction::IsZero => 0x15,
            Instruction::MLoad => 0x51,
            Instruction::MStore => 0x52,
            Instruction::Jump => 0x56,
            Instruction::JumpI => 0x57,
            Instruction::Push(n) => 0x60 + n - 1,
            Instruction::Dup(n) => 0x80 + n - 1,
            Instruction::Lit(x) => {
                match x {
                    Byte::C(x) => x,
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
            x => Byte::C(x.into()),
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

        match x {
            0x00 => Instruction::Stop,
            0x01 => Instruction::Add,
            0x03 => Instruction::Sub,
            0x15 => Instruction::IsZero,
            0x51 => Instruction::MLoad,
            0x52 => Instruction::MStore,
            0x56 => Instruction::Jump,
            0x57 => Instruction::JumpI,
            x => Instruction::Lit(Byte::C(x))
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
