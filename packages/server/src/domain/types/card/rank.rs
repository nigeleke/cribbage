use serde::{Deserialize, Serialize};

/// A card rank used for ordering/comparison.
///
/// Values range from `1` (Ace) to `13` (King).
///
/// # Examples
///
/// ```
/// # use my_crate::{Face, Rank};
/// assert_eq!(Face::Ace.rank(),   Rank(1));
/// assert_eq!(Face::King.rank(),  Rank(13));
///
/// let mut ranks = vec![Rank(Face::Ten.rank()), Face::Ace.rank(), Face::Queen.rank()];
/// ranks.sort();
/// assert_eq!(ranks, [Rank(1), Rank(10), Rank(12)]);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
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
