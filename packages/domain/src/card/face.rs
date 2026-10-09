use strum::{EnumIter, FromRepr};

use super::rank::Rank;
use super::value::Value;

/// The face of a playing card (Ace through King).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, FromRepr)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Face {
    #[doc(hidden)] Ace,
    #[doc(hidden)] Two,
    #[doc(hidden)] Three,
    #[doc(hidden)] Four,
    #[doc(hidden)] Five,
    #[doc(hidden)] Six,
    #[doc(hidden)] Seven,
    #[doc(hidden)] Eight,
    #[doc(hidden)] Nine,
    #[doc(hidden)] Ten,
    #[doc(hidden)] Jack,
    #[doc(hidden)] Queen,
    #[doc(hidden)] King,
}

impl Face {
    /// Returns the rank used for ordering (Ace low, King high).
    #[inline]
    pub const fn rank(self) -> Rank {
        match self {
            Self::Ace => Rank::new(0),
            Self::Two => Rank::new(1),
            Self::Three => Rank::new(2),
            Self::Four => Rank::new(3),
            Self::Five => Rank::new(4),
            Self::Six => Rank::new(5),
            Self::Seven => Rank::new(6),
            Self::Eight => Rank::new(7),
            Self::Nine => Rank::new(8),
            Self::Ten => Rank::new(9),
            Self::Jack => Rank::new(10),
            Self::Queen => Rank::new(11),
            Self::King => Rank::new(12),
        }
    }

    /// Returns the point or face value.
    ///
    /// - Ace = 1 (low)
    /// - 2–10 = face value
    /// - Jack/Queen/King = 10
    #[inline]
    pub const fn value(self) -> Value {
        match self {
            Self::Ace => Value::new(1),
            Self::Two => Value::new(2),
            Self::Three => Value::new(3),
            Self::Four => Value::new(4),
            Self::Five => Value::new(5),
            Self::Six => Value::new(6),
            Self::Seven => Value::new(7),
            Self::Eight => Value::new(8),
            Self::Nine => Value::new(9),
            Self::Ten | Face::Jack | Face::Queen | Face::King => Value::new(10),
        }
    }
}

impl std::fmt::Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Face::Ace => "A",
            Face::Two => "2",
            Face::Three => "3",
            Face::Four => "4",
            Face::Five => "5",
            Face::Six => "6",
            Face::Seven => "7",
            Face::Eight => "8",
            Face::Nine => "9",
            Face::Ten => "T",
            Face::Jack => "J",
            Face::Queen => "Q",
            Face::King => "K",
        })
    }
}
