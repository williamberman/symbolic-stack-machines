use super::word::Word;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Constraint {
    Eq(Box<Word>, Box<Word>),
    Neq(Box<Constraint>),
}

impl std::ops::Not for Constraint {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Constraint::Neq(x) => *x,
            x => Constraint::Neq(Box::new(x)),
        }
    }
}

impl Constraint {
    pub fn ite(self, then: Word, xelse: Word) -> Word {
        Word::Ite(Box::new(self), Box::new(then), Box::new(xelse))
    }
}
