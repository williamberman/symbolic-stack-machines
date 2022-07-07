use crate::val::{byte::Byte, word::{Word, BYTES_IN_WORD}};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Calldata {
    inner: Vec<Byte>,
}

impl Calldata {
    pub fn new(data: Vec<Byte>) -> Self {
        Self { inner: data }
    }

    pub fn size(&self) -> Word {
        self.inner.len().into()
    }

    pub fn read_word(&self, idx: Word) -> Word {
        let idx_unwrapped: usize = idx.into();
        Word::from_bytes_vec(&self.inner, idx_unwrapped, BYTES_IN_WORD, true)
    }
}

impl From<Vec<Byte>> for Calldata {
    fn from(data: Vec<Byte>) -> Self {
        Self::new(data)
    }
}

impl From<Vec<u8>> for Calldata {
    fn from(data: Vec<u8>) -> Self {
        let d: Vec<Byte> = data.into_iter().map(|x| x.into()).collect();
        d.into()
    }
}
