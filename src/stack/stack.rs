use im::Vector;

use super::{
    record::{StackOpRecord, StackRecord},
    StackVal,
};

#[derive(Clone, Default)]
pub struct Stack {
    inner: Vector<StackVal>,
}

impl Stack {
    pub fn new(init: Vector<StackVal>) -> Self {
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
        let mut inner = self.inner.clone();

        for c in r.changed {
            match c {
                StackOpRecord::Pop => {
                    inner.pop_back();
                }
                StackOpRecord::Push(x) => {
                    inner.push_back(x);
                }
            };
        }

        Self {
            inner,
        }
    }
}
