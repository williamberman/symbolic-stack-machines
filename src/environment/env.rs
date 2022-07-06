use crate::val::word::Word;

use super::record::EnvRecord;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Env {
    pub call_value: Word
}

impl Env {
    pub fn apply(&self, _r: EnvRecord) -> Self {
        self.clone()
    }
}
