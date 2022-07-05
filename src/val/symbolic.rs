use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Symbolic<T>
where
    T: std::ops::Add
        + std::ops::Add<Output = T>
        + std::ops::Sub
        + std::ops::Sub<Output = T>
        + Debug,
{
    C(T),
    S(String),
    Add(Box<Symbolic<T>>, Box<Symbolic<T>>),
    Sub(Box<Symbolic<T>>, Box<Symbolic<T>>),
}

impl<T> std::ops::Add for Symbolic<T>
where
    T: std::ops::Add
        + std::ops::Add<Output = T>
        + std::ops::Sub
        + std::ops::Sub<Output = T>
        + Debug,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::C(l), Self::C(r)) => Self::C(l + r),
            (l, r) => Self::Add(Box::new(l), Box::new(r)),
        }
    }
}

impl<T> std::ops::Sub for Symbolic<T>
where
    T: std::ops::Add
        + std::ops::Add<Output = T>
        + std::ops::Sub
        + std::ops::Sub<Output = T>
        + Debug,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::C(l), Self::C(r)) => Self::C(l - r),
            (l, r) => Self::Sub(Box::new(l), Box::new(r)),
        }
    }
}

// TODO(will) - this should directly be implemented via into
impl<T> Symbolic<T>
where
    T: std::ops::Add
        + std::ops::Add<Output = T>
        + std::ops::Sub
        + std::ops::Sub<Output = T>
        + Debug,
{
    pub fn concrete(self) -> T {
        if let Self::C(x) = self {
            return x;
        }

        panic!("invalid symbolic value {:?}", self);
    }
}

impl<T> From<T> for Symbolic<T>
where
    T: std::ops::Add
        + std::ops::Add<Output = T>
        + std::ops::Sub
        + std::ops::Sub<Output = T>
        + Debug,
{
    fn from(x: T) -> Self {
        Self::C(x)
    }
}
