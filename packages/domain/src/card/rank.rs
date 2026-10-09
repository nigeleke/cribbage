/// A card rank used for ordering/comparison.
///
/// Values range from `1` (Ace) to `13` (King).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Rank(u8);

impl Rank {
    pub(super) const fn new(value: u8) -> Self {
        Self(value)
    }
}

impl std::ops::Sub<Self> for Rank {
    type Output = i8;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as i8 - rhs.0 as i8
    }
}
