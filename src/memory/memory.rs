use im::Vector;

use crate::val::{byte::{Byte, ZERO_BYTE}, word::Word};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
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
        let xidx: usize = idx.into();

        // TODO(will) - What are the actual EVM semantics for memory extension
        if self.inner.len() < xidx + 32 {
            let n_additional_bytes_needed = xidx + 32 - self.inner.len();
            let iter = (0..n_additional_bytes_needed).map(|_| { ZERO_BYTE.clone() });
            self.inner.extend(iter);
        }

        Word::write_bytes(&mut self.inner, xidx, val.into());
    }

    fn read_byte_inner(&self, idx: usize) -> Option<&Byte> {
        self.inner.get(idx)
    }
}
