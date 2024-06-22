use serde::{Serialize, Deserialize};

/// The points score for a player.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Points(usize);

impl Points {
    #[cfg(test)]
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}

pub trait HasPoints {
    fn points(&self) -> Points;
}

impl From<usize> for Points {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for Points {
    type Output = Points;

    fn add(self, rhs: Self) -> Self::Output {
        Points(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Points {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Rem for Points {
    type Output = Points;

    fn rem(self, rhs: Self) -> Self::Output {
        Points(self.0 % rhs.0)
    }
}

impl std::iter::Sum for Points {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Points::default(), |acc, i| acc + i)
    }
}