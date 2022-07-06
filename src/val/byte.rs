#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Byte {
    C(u8),
    S(String)
}

impl Into<u8> for Byte {
    fn into(self) -> u8 {
        if let Self::C(x) = self {
            return x;
        }

        panic!("invalid symbolic value {:?}", self);
    }
}

impl From<u8> for Byte {
    fn from(x: u8) -> Self {
        Byte::C(x)
    }
}
