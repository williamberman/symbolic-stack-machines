use super::{
    record::{StackOpRecord, StackRecord},
    StackVal,
};

#[derive(Clone, Default)]
pub struct Stack {
    inner: Vec<StackVal>,
}

impl Stack {
    pub fn new(init: Vec<StackVal>) -> Self {
        Self {
            inner: init,
        }
    }

    pub fn peek(&self, idx: usize) -> Option<&StackVal> {
        let last_idx = self.inner.len() - 1;
        let get_idx = last_idx - idx;

        self.inner.get(get_idx)
    }

    pub fn apply(&self, r: StackRecord) -> Self {
        // TODO(will) - we should use a copy on write data structure
        let mut inner = self.inner.clone();

        for c in r.changed {
            match c {
                StackOpRecord::Pop => {
                    inner.pop();
                }
                StackOpRecord::Push(x) => {
                    inner.push(x);
                }
            };
        }

        Self {
            inner,
        }
    }
}
