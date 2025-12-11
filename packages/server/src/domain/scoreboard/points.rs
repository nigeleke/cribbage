use serde::{Deserialize, Serialize};

/// Represents a point value in the game.
///
/// Wraps a `usize` and provides ordering and comparison operations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Points(usize);

impl Points {
    /// Returns the internal numeric value of the points.
    #[must_use]
    pub fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add<Points> for Points {
    type Output = Points;

    fn add(self, rhs: Points) -> Self::Output {
        Points::from(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Points {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::iter::Sum<Self> for Points {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.map(|v| v.0).sum::<usize>().into()
    }
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn points_will_be_displayed_as_numeric_value() {
        let points = Points::from(42);
        assert_eq!(points.to_string(), "42");
    }
}
