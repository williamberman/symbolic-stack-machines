use im::Vector;

use crate::stack::StackVal;

use super::{
    record::{MemOpRecord, MemRecord},
    val::MemVal,
};

#[derive(Clone, Default)]
pub struct Memory {
    inner: Vector<MemVal>,
}

impl Memory {
    pub fn new(init: Vector<MemVal>) -> Self {
        Self {
            inner: init,
        }
    }

    pub fn read_word(&self, idx: StackVal) -> Option<StackVal> {
        let idx_unwrapped = Into::<usize>::into(idx);

        // TODO(will): Check endianness/byte ordering
        let mut bytes: [u8; 8] = [0; 8];

        for i in 0..=7 {
            let byte: u8 = (*self.read_byte_inner(idx_unwrapped + i)?).into();
            bytes[i] = byte
        }

        Some(u64::from_be_bytes(bytes).into())
    }

    pub fn read_byte(&self, idx: StackVal) -> Option<&MemVal> {
        self.read_byte_inner(Into::<usize>::into(idx))
    }

    fn read_byte_inner(&self, idx: usize) -> Option<&MemVal> {
        self.inner.get(idx)
    }

    pub fn apply(&self, r: MemRecord) -> Self {
        let mut inner = self.inner.clone();

        for c in r.changed {
            match c {
                MemOpRecord::Write(idx, val) => {
                    // TODO(will): Check endianness/byte ordering
                    let idx_usize = Into::<usize>::into(idx);
                    let val_unwrapped = Into::<u64>::into(val).to_be_bytes();

                    for i in 0..=7 {
                        inner[idx_usize + i] = val_unwrapped[i].into();
                    }
                }
            }
        }

        Self {
            inner,
        }
    }
}
