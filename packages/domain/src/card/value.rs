use crate::Face;

/// Point value of a card.
///
/// Standard mapping:
/// - Ace → 1
/// - 2–10 → face value
/// - Jack / Queen / King → 10
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Value(usize);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<&Face> for Value {
    fn from(value: &Face) -> Self {
        match value {
            Face::Ace => Self(1),
            Face::Two => Self(2),
            Face::Three => Self(3),
            Face::Four => Self(4),
            Face::Five => Self(5),
            Face::Six => Self(6),
            Face::Seven => Self(7),
            Face::Eight => Self(8),
            Face::Nine => Self(9),
            Face::Ten | Face::Jack | Face::Queen | Face::King => Self(10),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::iter::Sum<Self> for Value {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|value| value.0).sum())
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn from_face() {
        Face::iter()
            .zip((1..10).chain(std::iter::repeat_n(10, 4)).map(Value))
            .for_each(|(actual, expected)| assert_eq!(actual.value(), expected));
    }

    #[test]
    fn add() {
        assert_eq!(Value(3) + Value(2), Value(5))
    }

    #[test]
    fn add_assign() {
        let mut value = Value(3);
        value += Value(2);
        assert_eq!(value, Value(5))
    }

    #[test]
    fn sum() {
        let values = (0..10).collect::<Vec<_>>();
        let expected = values.iter().sum();
        assert_eq!(
            values.iter().map(|v| Value(*v)).sum::<Value>(),
            Value(expected)
        )
    }
}
