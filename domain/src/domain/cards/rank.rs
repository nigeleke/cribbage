use serde::{Deserialize, Serialize};

/// The rank of a Card. Ace(1) to King(13).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Rank(usize);

impl Rank {
    pub const fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
