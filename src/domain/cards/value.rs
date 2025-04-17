use serde::{Deserialize, Serialize};

/// The value of a Card. 1 to 10.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(usize);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value(value)
    }
}

impl std::ops::Deref for Value {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Value(self.0 + rhs.0)
    }
}

// impl std::ops::AddAssign for Value {
//     fn add_assign(&mut self, rhs: Self) {
//         self.0 += rhs.0;
//     }
// }

impl std::iter::Sum<Value> for Value {
    fn sum<I: Iterator<Item = Value>>(iter: I) -> Self {
        iter.map(|v| v.0).sum::<usize>().into()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn array_of_values_can_be_summed() {
        let values = Vec::from([3, 4].map(Value::from));
        let sum: Value = values.into_iter().sum();
        assert_eq!(sum, Value::from(7));
    }
}
