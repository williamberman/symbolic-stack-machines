use super::{Mem, MemoryResult, RWMem, ReadOnlyMem, WriteableMem};
use std::fmt::Debug;

pub trait MemVal: Default + Clone + Debug {}

#[derive(Clone, Debug)]
pub struct MemConcreteIndex<MV: MemVal> {
    inner: Vec<MV>,
}

impl<MV: MemVal> Mem for MemConcreteIndex<MV> {
    type MemVal = MV;
    type Index = usize;
}

impl<MV: MemVal> ReadOnlyMem for MemConcreteIndex<MV> {
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>> {
        Ok(Some(
            self.inner
                .get(idx)
                .map(|x| (*x).clone())
                .unwrap_or_default(),
        ))
    }
}

impl<MV: MemVal> WriteableMem for MemConcreteIndex<MV> {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self> {
        let mut x = Self {
            inner: self.inner.clone(),
        };

        let min_len = idx + 1;

        if x.inner.len() < min_len {
            x.inner.resize(min_len, Self::MemVal::default());
        }

        x.inner[idx] = val;

        Ok(x)
    }
}

impl<MV: MemVal> MemConcreteIndex<MV> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { inner: vec![] }
    }
}

impl<MV: MemVal> RWMem for MemConcreteIndex<MV> {}
