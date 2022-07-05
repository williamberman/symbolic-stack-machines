use im::Vector;
use primitive_types::U256;

use crate::instructions::Instruction;

use super::byte::Byte;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Word(U256);

impl<T> From<T> for Word
where
    U256: From<T>,
{
    fn from(x: T) -> Self {
        Self::from(U256::from(x))
    }
}

impl Into<U256> for Word {
    fn into(self) -> U256 {
        self.0
    }
}

impl Into<usize> for Word {
    fn into(self) -> usize {
        self.0.as_usize()
    }
}

impl Into<[Instruction; 32]> for Word {
    fn into(self) -> [Instruction; 32] {
        let mut rv = [0; 32];
        self.0.to_big_endian(&mut rv);
        rv.map(|x| Instruction::Lit(x))
    }
}

impl Word {
    pub fn _eq(&self, other: &Self) -> Self {
        if self == other {
            Self::true_word()
        } else {
            Self::false_word()
        }
    }

    pub fn ite(&self, then: Self, xelse: Self) -> Self {
        if *self == Self::true_word() {
            then
        } else {
            xelse
        }
    }
}

impl std::ops::Add for Word {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Word {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Word {
    // Create a word from 32 bytes starting at idx in bs
    pub fn from_bytes_vector<T: Into<u8> + Clone>(bs: &Vector<T>, idx: usize) -> Self {
        Self::from_bytes(|offset| bs.get(idx + offset).cloned())
    }

    // Create a word from 32 bytes starting at idx in bs
    pub fn from_bytes_vec<T: Into<u8> + Clone>(bs: &Vec<T>, idx: usize) -> Self {
        Self::from_bytes(|offset| bs.get(idx + offset).cloned())
    }

    fn from_bytes<T: Into<u8> + Clone, F: Fn(usize) -> Option<T>>(f: F) -> Self {
        let mut bytes: [u8; 32] = [0; 32];

        for i in 0..=31 {
            let byte: u8 = f(i).unwrap().into();
            bytes[i] = byte;
        }

        Self::from(U256::from(bytes))
    }

    pub fn write_bytes(bs: &mut Vector<Byte>, idx: usize, val: Word) {
        let u256: U256 = val.into();

        for i in 0..=31 {
            bs[idx + i] = u256.byte(i).into();
        }
    }

    pub fn zero() -> Self {
        Word(U256::zero())
    }

    pub fn one() -> Self {
        Word(U256::one())
    }

    pub fn false_word() -> Self {
        Self::zero()
    }

    pub fn true_word() -> Self {
        Self::one()
    }

    pub fn constant_instruction<T>(val: T) -> [Instruction; 32] where Self: From<T> {
        Self::from(val).into()
    }
}
