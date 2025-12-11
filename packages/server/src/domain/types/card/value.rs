/// Point value of a card.
///
/// Standard mapping:
/// - Ace → 1
/// - 2–10 → face value
/// - Jack / Queen / King → 10
///
/// # Examples
///
/// ```
/// # use my_crate::{Face, Value};
/// assert_eq!(Face::Ace.value(),   Value(1));
/// assert_eq!(Face::Ten.value(),   Value(10));
/// assert_eq!(Face::King.value(),  Value(10));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Value(usize);

impl Value {
    /// Returns the underlying numeric value.
    #[inline(always)]
    #[must_use]
    pub fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::iter::Sum<Self> for Value {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.map(|v| v.0).sum::<usize>().into()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use super::*;

    #[test]
    fn array_of_values_can_be_summed() {
        let values = Vec::from([3, 4].map(Value::from));
        let sum: Value = values.into_iter().sum();
        assert_eq!(sum, Value::from(7));
    }

    #[test]
    fn value_can_be_displayed() {
        let value = Value::from(42);
        assert_eq!(value.to_string(), "42".to_string());
    }
}
