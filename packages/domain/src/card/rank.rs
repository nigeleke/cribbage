use crate::Face;

/// A card rank used for ordering/comparison.
///
/// Values range from `1` (Ace) to `13` (King).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Rank(u8);

impl From<&Face> for Rank {
    fn from(value: &Face) -> Self {
        match value {
            Face::Ace => Self(0),
            Face::Two => Self(1),
            Face::Three => Self(2),
            Face::Four => Self(3),
            Face::Five => Self(4),
            Face::Six => Self(5),
            Face::Seven => Self(6),
            Face::Eight => Self(7),
            Face::Nine => Self(8),
            Face::Ten => Self(9),
            Face::Jack => Self(10),
            Face::Queen => Self(11),
            Face::King => Self(12),
        }
    }
}

impl std::ops::Sub<Self> for Rank {
    type Output = i8;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as i8 - rhs.0 as i8
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn from_face() {
        Face::iter()
            .zip((0..13).map(Rank))
            .for_each(|(actual, expected)| assert_eq!(actual.rank(), expected));
    }
}
