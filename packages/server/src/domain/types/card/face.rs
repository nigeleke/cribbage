use serde::{Deserialize, Serialize};
use strum::EnumIter;

use super::{rank::Rank, value::Value};
use crate::domain::CardsError;

/// The face of a playing card (Ace through King).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
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
    /// Returns `true` if this face is a Jack.
    #[must_use]
    #[inline]
    pub const fn is_jack(&self) -> bool {
        matches!(self, Self::Jack)
    }

    /// Returns the rank used for ordering (Ace low, King high).
    #[must_use]
    pub fn rank(&self) -> Rank {
        let rank = match self {
            Self::Ace => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
        };

        Rank::from(rank)
    }

    /// Returns the point or face value.
    ///
    /// - Ace = 1 (low)
    /// - 2–10 = face value
    /// - Jack/Queen/King = 10
    #[must_use]
    pub fn value(&self) -> Value {
        let value = match self {
            Self::Ace => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten | Self::Jack | Self::Queen | Self::King => 10,
        };

        Value::from(value)
    }
}

impl TryFrom<char> for Face {
    type Error = CardsError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            other => Err(CardsError::InvalidCard(format!("Invalid face: {other}"))),
        }
    }
}

impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Ace => "A",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "T",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
        };
        s.fmt(f)
    }
}

#[cfg(test)]
#[coverage(off)]
pub mod test {
    use super::*;

    #[test]
    fn face_has_rank_value_display_string_and_name() {
        let faces = [
            (Face::Ace, 1, 1, "A"),
            (Face::Two, 2, 2, "2"),
            (Face::Three, 3, 3, "3"),
            (Face::Four, 4, 4, "4"),
            (Face::Five, 5, 5, "5"),
            (Face::Six, 6, 6, "6"),
            (Face::Seven, 7, 7, "7"),
            (Face::Eight, 8, 8, "8"),
            (Face::Nine, 9, 9, "9"),
            (Face::Ten, 10, 10, "T"),
            (Face::Jack, 11, 10, "J"),
            (Face::Queen, 12, 10, "Q"),
            (Face::King, 13, 10, "K"),
        ];

        for (face, rank, value, display_string) in faces {
            assert_eq!(face.rank(), Rank::from(rank));
            assert_eq!(face.value(), Value::from(value));
            assert_eq!(face.to_string(), display_string);
        }
    }
}
