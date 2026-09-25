use std::ptr::write;

use strum::EnumIter;

use super::{Rank, Value};

/// The face of a playing card (Ace through King).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
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
    pub fn rank(&self) -> Rank {
        Rank::from(self)
    }

    /// Returns the point or face value.
    ///
    /// - Ace = 1 (low)
    /// - 2–10 = face value
    /// - Jack/Queen/King = 10
    #[inline]
    pub fn value(&self) -> Value {
        Value::from(self)
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
