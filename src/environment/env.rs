use crate::val::word::Word;

use super::record::EnvRecord;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Env {
    pub call_value: Word,
    pub call_data_size: Word,
    pub revert_offset: Option<Word>,
    pub revert_length: Option<Word>
}

impl Env {
    pub fn apply(&self, _r: EnvRecord) -> Self {
        self.clone()
    }
}
