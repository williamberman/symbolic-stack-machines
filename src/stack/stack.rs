use im::Vector;

use super::StackVal;

#[derive(Clone, Default)]
pub struct Stack {
    inner: Vector<StackVal>,
}

impl Stack {
    pub fn new(init: Vector<StackVal>) -> Self {
        Self { inner: init }
    }

    pub fn pop(&mut self) -> Option<StackVal> {
        self.inner.pop_back()
    }

    pub fn push(&mut self, val: StackVal) {
        self.inner.push_back(val)
    }
}
