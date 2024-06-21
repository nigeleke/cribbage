use serde::{Serialize, Deserialize};

/// The value of a Card. 1 to 10.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(usize);

impl Value {
    pub fn new(value: usize) -> Self {
        Value(value)
    }
}
pub trait HasValue {
    fn value(&self) -> Value;
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Value(self.0 + rhs.0)
    }
}

impl std::iter::Sum for Value {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Value::new(0), |acc, x| acc + x)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
