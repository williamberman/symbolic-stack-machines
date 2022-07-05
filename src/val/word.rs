#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Word(u64);

pub static ZERO_WORD: Word = Word(0);
pub static ONE_WORD: Word = Word(1);

static FALSE_WORD: Word = ZERO_WORD;
static TRUE_WORD: Word = ONE_WORD;

impl From<u64> for Word {
    fn from(x: u64) -> Self {
        Self(x)
    }
}

impl Into<u64> for Word {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<usize> for Word {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Word {
    pub fn _eq(&self, other: &Self) -> Self {
        if self == other {
            TRUE_WORD
        } else {
            FALSE_WORD
        }
    }

    pub fn ite(&self, then: Self, xelse: Self) -> Self {
        if *self == TRUE_WORD {
            then
        } else {
            xelse
        }
    }
}

impl std::ops::Add for Word {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Word {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
