/// Represents a point value in the game.
///
/// Wraps a `usize` and provides ordering and comparison operations.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Points(usize);

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Points {
    type Output = Points;

    fn add(self, rhs: Points) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Points {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl std::iter::Sum<Self> for Points {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self(iter.map(|value| value.0).sum())
    }
}

impl std::fmt::Debug for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_points_are_zero() {
        assert_eq!(Points::default(), Points(0));
    }

    #[test]
    fn points_can_be_added() {
        assert_eq!(Points(10) + Points(5), Points(15));
    }

    #[test]
    fn points_can_be_added_assign() {
        let mut points = Points(10);
        points += Points(5);
        assert_eq!(points, Points(15));
    }

    #[test]
    fn points_can_be_summed() {
        let points = [Points(2), Points(5), Points(7)];
        assert_eq!(points.into_iter().sum::<Points>(), Points(14));
    }

    #[test]
    fn sum_of_no_points_is_zero() {
        let points = std::iter::empty::<Points>();
        assert_eq!(points.sum::<Points>(), Points::default());
    }

    #[test]
    fn adding_zero_does_not_change_points() {
        assert_eq!(Points(15) + Points::default(), Points(15));
    }
}
