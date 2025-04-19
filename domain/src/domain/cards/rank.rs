/// The rank of a Card. Ace(1) to King(13).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rank(usize);

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Rank {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
