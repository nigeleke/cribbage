use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

use crate::domain::CardsError;

/// The four suits in a standard French playing card deck.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter, AsRefStr)]
#[rustfmt::skip]
pub enum Suit {
    #[doc(hidden)] Hearts,
    #[doc(hidden)] Clubs,
    #[doc(hidden)] Diamonds,
    #[doc(hidden)] Spades,
}

impl TryFrom<char> for Suit {
    type Error = CardsError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'H' => Ok(Self::Hearts),
            'C' => Ok(Self::Clubs),
            'D' => Ok(Self::Diamonds),
            'S' => Ok(Self::Spades),
            other => Err(CardsError::InvalidCard(format!("Invalid suit: {other}"))),
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Hearts => "H",
            Self::Clubs => "C",
            Self::Diamonds => "D",
            Self::Spades => "S",
        };
        s.fmt(f)
    }
}

#[cfg(test)]
#[coverage(off)]
pub mod test {
    use super::*;

    #[test]
    fn suit_has_display_string() {
        let suits = [
            (Suit::Hearts, "H"),
            (Suit::Clubs, "C"),
            (Suit::Diamonds, "D"),
            (Suit::Spades, "S"),
        ];

        for (suit, display_string) in suits {
            assert_eq!(suit.to_string(), display_string);
        }
    }
}
