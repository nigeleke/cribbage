use serde::{Deserialize, Serialize};

/// The rank of a Card. Ace(1) to King(13).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Rank(usize);

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Sub for Rank {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}
