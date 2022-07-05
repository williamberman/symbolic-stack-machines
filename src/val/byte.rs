#[derive(Clone, Copy)]
pub struct Byte(u8);

impl Into<u8> for Byte {
    fn into(self) -> u8 {
        self.0
    }
}

impl From<u8> for Byte {
    fn from(x: u8) -> Self {
        Byte(x)
    }
}
