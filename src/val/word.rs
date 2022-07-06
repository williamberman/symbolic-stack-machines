use im::Vector;
use primitive_types::U256;

use crate::instructions::Instruction;

use super::{byte::Byte, constraint::Constraint};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Word {
    C(U256),
    S(String),
    Add(Box<Word>, Box<Word>),
    Sub(Box<Word>, Box<Word>),
    Ite(Box<Constraint>, Box<Word>, Box<Word>),
    Concat([Byte; 32]),
}

impl From<u32> for Word {
    fn from(x: u32) -> Self {
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
    ) -> Self {
        Self::from_bytes(len, |offset| bs.get(idx + offset).cloned())
    }

    pub fn from_bytes_vec<T: Into<Byte> + Clone>(bs: &Vec<T>, idx: usize, len: usize) -> Self {
        Self::from_bytes(len, |offset| bs.get(idx + offset).cloned())
    }

    fn from_bytes<T: Into<Byte> + Clone, F: Fn(usize) -> Option<T>>(len: usize, f: F) -> Self {
        let mut bytes: [u8; 32] = [0; 32];
        // TODO(will): Should be better way to initialize array
        let mut sym_bytes: [Byte; 32] = bytes.map(|x| { x.into() });

        let mut all_concrete = true;

        for i in 0..=(len - 1) {
            let write_idx = 32 - len + i;

            let sym_byte: Byte = f(i).unwrap().into();

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

    pub fn constant_instruction<T>(val: T) -> [Instruction; 32]
    where
        Self: From<T>,
    {
        Self::from(val).into()
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

mod tests {
    use crate::val::word::Word;

    static BS: [u8; 51] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50,
    ];

    #[test]
    pub fn word_from_bytes_full() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 0, 32);
        let expected = Word::from([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_offset() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 1, 32);
        let expected = Word::from([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_offset_2() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 12, 32);
        let expected = Word::from([
            12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
            34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_len() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 0, 31);
        let expected = Word::from([
            0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26, 27, 28, 29, 30,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_len_2() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 0, 15);
        let expected = Word::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_mixed() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 1, 15);
        let expected = Word::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
            12, 13, 14, 15,
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn word_from_bytes_mixed_2() {
        let actual = Word::from_bytes_vec(&Vec::from(BS), 10, 10);
        let expected = Word::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14,
            15, 16, 17, 18, 19,
        ]);

        assert_eq!(actual, expected);
    }
}
