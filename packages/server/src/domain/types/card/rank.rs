use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Rank(usize);

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Add<usize> for Rank {
    type Output = Rank;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Sub for Rank {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}
