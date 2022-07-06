use im::Vector;

use crate::val::{byte::Byte, word::Word};

#[derive(Clone, Default, Debug)]
pub struct Memory {
    inner: Vector<Byte>,
}

impl Memory {
    pub fn new(init: Vector<Byte>) -> Self {
        Self { inner: init }
    }

    pub fn read_word(&self, idx: Word) -> Word {
        let idx_unwrapped = Into::<usize>::into(idx);

        Word::from_bytes_vector(&self.inner, idx_unwrapped, 32)
    }

    pub fn read_byte(&self, idx: Word) -> Option<&Byte> {
        self.read_byte_inner(Into::<usize>::into(idx))
    }

    pub fn write_word(&mut self, idx: Word, val: Word) {
        Word::write_bytes(&mut self.inner, idx.into(), val.into());
    }

    fn read_byte_inner(&self, idx: usize) -> Option<&Byte> {
        self.inner.get(idx)
    }
}
