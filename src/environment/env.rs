use super::record::EnvRecord;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env {}

impl Env {
    pub fn apply(&self, _r: EnvRecord) -> Self {
        self.clone()
    }
}
