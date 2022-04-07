pub mod error;
pub mod symbolic_concrete_index;
pub mod symbolic_symbolic_index;

use error::MemoryError;

pub type MemoryResult<T> = Result<T, MemoryError>;

pub trait Mem: Sized + std::fmt::Debug {
    type Index;
    type MemVal;
}

pub trait ReadOnlyMem: Mem {
    fn read(&self, idx: Self::Index) -> MemoryResult<Option<Self::MemVal>>;
}

pub trait WriteableMem: Mem {
    fn write(&self, idx: Self::Index, val: Self::MemVal) -> MemoryResult<Self>;
}

pub trait RWMem: ReadOnlyMem + WriteableMem {
}

pub type MemorySlotChange<Idx, MemVal> = (Idx, MemVal);
pub enum MemOpRecord<I, V> {
    Write(MemorySlotChange<I, V>),
}
pub struct MemRecord<M: Mem> {
    pub diff: Vec<MemOpRecord<M::Index, M::MemVal>>,
}

impl<M> MemRecord<M>
where
    M: WriteableMem,
{
    pub fn apply(self, memory: M) -> MemoryResult<M> {
        let final_mem = self.diff.into_iter().fold(
            Ok(memory),
            |cur_mem: MemoryResult<M>, r| -> MemoryResult<M> {
                match cur_mem {
                    Ok(m) => {
                        if let MemOpRecord::Write(r) = r {
                            let idx = r.0;
                            let new_val = r.1;
                            m.write(idx, new_val)
                        } else {
                            Ok(m)
                        }
                    }
                    Err(e) => Err(e),
                }
            },
        );
        final_mem
    }
}
