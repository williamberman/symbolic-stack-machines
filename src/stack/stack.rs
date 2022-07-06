use im::Vector;
use crate::val::word::Word;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Stack {
    inner: Vector<Word>,
}

impl Stack {
    pub fn new(init: Vector<Word>) -> Self {
        Self { inner: init }
    }

    pub fn pop(&mut self) -> Option<Word> {
        self.inner.pop_back()
    }

    pub fn push(&mut self, val: Word) {
        self.inner.push_back(val)
    }

    pub fn peek(&self) -> Option<&Word> {
        self.inner.back()
    }

    pub fn peek_n(&self, n: usize) -> Option<&Word> {
        self.inner.get(self.inner.len() - n - 1)
    }
}
