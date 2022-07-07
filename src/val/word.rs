use im::Vector;
use primitive_types::U256;

use crate::{instructions::Instruction, utils::I256};

use super::{
    byte::{Byte, ZERO_BYTE},
    constraint::Constraint,
};

pub static BYTES_IN_WORD: usize = 32;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Word {
    C(U256),
    S(String),
    Add(Box<Word>, Box<Word>),
    Sub(Box<Word>, Box<Word>),
    Lt(Box<Word>, Box<Word>),
    Slt(Box<Word>, Box<Word>),
    Shr(Box<Word>, Box<Word>),
    Ite(Box<Constraint>, Box<Word>, Box<Word>),
    Concat([Byte; 32]),
}

impl Default for Word {
    fn default() -> Self {
        Word::zero()
    }
}

impl From<u32> for Word {
    fn from(x: u32) -> Self {
        Self::from(U256::from(x))
    }
}

impl From<i32> for Word {
    fn from(x: i32) -> Self {
        Self::from(U256::from(x))
    }
}

impl From<usize> for Word {
    fn from(x: usize) -> Self {
        Self::from(U256::from(x))
    }
}

impl From<[u8; 32]> for Word {
    fn from(x: [u8; 32]) -> Self {
        Self::from(U256::from(x))
    }
}

impl From<[Byte; 32]> for Word {
    fn from(x: [Byte; 32]) -> Self {
        Word::Concat(x)
    }
}

impl Into<U256> for Word {
    fn into(self) -> U256 {
        if let Self::C(x) = self {
            return x;
        }

        panic!("invalid symbolic value {:?}", self);
    }
}

impl From<U256> for Word {
    fn from(x: U256) -> Self {
        Self::C(x)
    }
}

impl Into<usize> for Word {
    fn into(self) -> usize {
        let x: U256 = self.into();
        x.as_usize()
    }
}

impl Into<[Instruction; 32]> for Word {
    fn into(self) -> [Instruction; 32] {
        let mut rv = [0; 32];
        let x: U256 = self.into();
        x.to_big_endian(&mut rv);
        rv.map(|x| Instruction::Lit(Byte::C(x)))
    }
}

impl Word {
    pub fn ite(&self, then: Self, xelse: Self) -> Self {
        if *self == Self::true_word() {
            then
        } else {
            xelse
        }
    }
}

impl Word {
    pub fn from_bytes_vector<T: Into<Byte> + Clone>(
        bs: &Vector<T>,
        idx: usize,
        len: usize,
        allow_index_past_end: bool,
    ) -> Self {
        Self::from_bytes(len, |offset| {
            if allow_index_past_end && idx + offset >= bs.len() {
                ZERO_BYTE.clone()
            } else {
                bs.get(idx + offset).unwrap().clone().into()
            }
        })
    }

    pub fn from_bytes_vec<T: Into<Byte> + Clone>(
        bs: &Vec<T>,
        idx: usize,
        len: usize,
        allow_index_past_end: bool,
    ) -> Self {
        Self::from_bytes(len, |offset| {
            if allow_index_past_end && idx + offset >= bs.len() {
                ZERO_BYTE.clone()
            } else {
                bs.get(idx + offset).unwrap().clone().into()
            }
        })
    }

    fn from_bytes<F: Fn(usize) -> Byte>(len: usize, f: F) -> Self {
        let mut bytes: [u8; 32] = [0; 32];
        // TODO(will): Should be better way to initialize array
        let mut sym_bytes: [Byte; 32] = bytes.map(|x| x.into());

        let mut all_concrete = true;

        for i in 0..=(len - 1) {
            let write_idx = 32 - len + i;

            let sym_byte = f(i);

            sym_bytes[write_idx] = sym_byte.clone();

            match sym_byte {
                Byte::C(x) => {
                    bytes[write_idx] = x;
                }
                Byte::S(_) => {
                    all_concrete = false;
                }
            }
        }

        if all_concrete {
            return Self::from(U256::from(bytes));
        }

        Word::from(sym_bytes)
    }

    pub fn write_bytes(bs: &mut Vector<Byte>, idx: usize, val: Word) {
        let u256: U256 = val.into();

        for i in 0..=31 {
            bs[idx + i] = u256.byte(i).into();
        }
    }

    pub fn zero() -> Self {
        Word::C(U256::zero())
    }

    pub fn one() -> Self {
        Word::C(U256::one())
    }

    pub fn false_word() -> Self {
        Self::zero()
    }

    pub fn true_word() -> Self {
        Self::one()
    }

    pub fn _eq(self, other: Self) -> Constraint {
        Constraint::Eq(Box::new(self), Box::new(other))
    }

    pub fn _eq_word(self, other: Self) -> Self {
        Constraint::Eq(Box::new(self), Box::new(other)).ite(Word::one(), Word::zero())
    }

    pub fn constant_instruction<T>(val: T) -> [Instruction; 32]
    where
        Self: From<T>,
    {
        Self::from(val).into()
    }

    pub fn _lt(self, other: Word) -> Word {
        match (self, other) {
            (Word::C(l), Word::C(r)) => {
                if l < r {
                    Word::one()
                } else {
                    Word::zero()
                }
            }
            (l, r) => Word::Lt(Box::new(l), Box::new(r)),
        }
    }

    pub fn _slt(self, other: Word) -> Word {
        match (self, other) {
            (Word::C(l), Word::C(r)) => {
                let op1: I256 = l.into();
                let op2: I256 = r.into();
            
                let rv = if op1.lt(&op2) {
                    U256::one()
                } else {
                    U256::zero()
                };

                rv.into()
            },
            (l, r) => Word::Slt(Box::new(l), Box::new(r))
        }
    }
}

impl std::ops::Add for Word {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::C(l), Self::C(r)) => Self::C(l + r),
            (l, r) => Self::Add(Box::new(l), Box::new(r)),
        }
    }
}

impl std::ops::Sub for Word {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::C(l), Self::C(r)) => Self::C(l - r),
            (l, r) => Self::Sub(Box::new(l), Box::new(r)),
        }
    }
}

impl std::ops::Shr for Word {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Word::C(value), Word::C(shift)) => {
                let w = if value == U256::zero() || shift >= U256::from(256) {
                    U256::zero()
                } else {
                    let shift: u64 = shift.as_u64();
                    value >> shift as usize
                };

                w.into()
            }
            (value, shift) => Self::Shr(Box::new(value), Box::new(shift)),
        }
    }
}

mod tests {
    use crate::val::word::Word;

    static BS: [u8; 51] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50,
    ];

    #[test]
    pub fn word_from_bytes_full() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 0, 32, false);
        let expected = Word::from([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_offset() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 1, 32, false);
        let expected = Word::from([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_offset_2() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 12, 32, false);
        let expected = Word::from([
            12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
            34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_len() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 0, 31, false);
        let expected = Word::from([
            0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26, 27, 28, 29, 30,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_len_2() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 0, 15, false);
        let expected = Word::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_mixed() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 1, 15, false);
        let expected = Word::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
            12, 13, 14, 15,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_mixed_2() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 10, 10, false);
        let expected = Word::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19,
        ]);

        assert_eq!(actual, expected);
    }
}
