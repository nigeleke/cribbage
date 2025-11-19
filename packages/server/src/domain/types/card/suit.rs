use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

use crate::domain::CardsError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, AsRefStr)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

impl Suit {
    pub fn name(&self) -> String {
        self.as_ref().to_string()
    }
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
    fn suit_has_display_string_and_name() {
        let suits = [
            (Suit::Hearts, "H", "Hearts"),
            (Suit::Clubs, "C", "Clubs"),
            (Suit::Diamonds, "D", "Diamonds"),
            (Suit::Spades, "S", "Spades"),
        ];

        for (suit, display_string, name) in suits {
            assert_eq!(suit.to_string(), display_string);
            assert_eq!(suit.name(), name);
        }
    }
}
