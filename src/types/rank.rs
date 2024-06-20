use serde::{Serialize, Deserialize};

/// The rank of a Card. Ace(1) to King(13).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(usize);

impl Rank {
    pub fn new(rank: usize) -> Self {
        Self(rank)
    }
}

pub trait HasRank {
    fn rank(&self) -> Rank;
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Sub for Rank {
    type Output = Rank;

    fn sub(self, rhs: Self) -> Self::Output {
        Rank(self.0 - rhs.0)
    }
}
