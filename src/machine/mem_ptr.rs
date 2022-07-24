use crate::val::word::Word;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct MemPtr {
    pub offset: Word,
    pub length: Word,
}
