use super::word::Word;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Byte {
    C(u8, Option<String>),
    S(String),
    Idx(Box<Word>, usize)
}

pub static ZERO_BYTE: Byte = Byte::C(0, None);

impl Into<u8> for Byte {
    fn into(self) -> u8 {
        if let Self::C(x, _) = self {
            return x;
        }

        panic!("invalid symbolic value {:?}", self);
    }
}

impl From<u8> for Byte {
    fn from(x: u8) -> Self {
        Byte::C(x, None)
    }
}

impl From<&str> for Byte {
    fn from(x: &str) -> Self {
        Byte::S(x.into())
    }
}

impl Byte {
    pub fn is_concrete(&self) -> bool {
        match self {
            Byte::C(_, _) => true,
            _ => false
        }
    }
}