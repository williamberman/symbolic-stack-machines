use im::Vector;
use primitive_types::U256;

use crate::instructions::Instruction;

use super::byte::Byte;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Word {
    C(U256),
    S(String),
    Add(Box<Word>, Box<Word>),
    Sub(Box<Word>, Box<Word>),
}

impl From<U256> for Word {
    fn from(x: U256) -> Self {
        Self::C(x)
    }
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

impl Into<U256> for Word {
    fn into(self) -> U256 {
        if let Word::C(x) = self {
            return x;
        }

        panic!("invalid symbolic value {:?}", self);
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
        match (self, rhs) {
            (Word::C(l), Word::C(r)) => Word::C(l + r),
            (l, r) => Word::Add(Box::new(l), Box::new(r)),
        }
    }
}

impl std::ops::Sub for Word {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Word::C(l), Word::C(r)) => Word::C(l - r),
            (l, r) => Word::Sub(Box::new(l), Box::new(r)),
        }
    }
}

impl Word {
    pub fn from_bytes_vector<T: Into<u8> + Clone>(bs: &Vector<T>, idx: usize, len: usize) -> Self {
        Self::from_bytes(len, |offset| bs.get(idx + offset).cloned())
    }

    pub fn from_bytes_vec<T: Into<u8> + Clone>(bs: &Vec<T>, idx: usize, len: usize) -> Self {
        Self::from_bytes(len, |offset| bs.get(idx + offset).cloned())
    }

    // Create a word from len bytes starting in bs
    fn from_bytes<T: Into<u8> + Clone, F: Fn(usize) -> Option<T>>(len: usize, f: F) -> Self {
        let mut bytes: [u8; 32] = [0; 32];

        for i in 0..=(len - 1) {
            let byte: u8 = f(i).unwrap().into();
            bytes[32 - len + i] = byte;
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

    pub fn constant_instruction<T>(val: T) -> [Instruction; 32]
    where
        Self: From<T>,
    {
        Self::from(val).into()
    }
}

mod tests {
    use super::Word;

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
