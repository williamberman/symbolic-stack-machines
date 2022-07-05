use super::symbolic::Symbolic;

pub type Byte = Symbolic<u8>;

// TODO(will) - this should be derived from the trait
impl Into<u8> for Byte {
    fn into(self) -> u8 {
        self.concrete()
    }
}
