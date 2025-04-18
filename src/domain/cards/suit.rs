use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

/// A Card suit.
#[derive(Clone, Copy, Debug, Deserialize, Sequence, Serialize, PartialEq)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Hearts => "H",
            Suit::Clubs => "C",
            Suit::Diamonds => "D",
            Suit::Spades => "S",
        };
        s.fmt(f)
    }
}

impl From<char> for Suit {
    fn from(value: char) -> Self {
        match value {
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'S' => Suit::Spades,
            _ => panic!("Unknown suit"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_suit_from_valid_char() {
        for c in "HCDS".as_bytes() {
            let _ = Suit::from(*c as char);
        }
    }

    #[test]
    #[should_panic]
    fn cannot_create_suit_from_invalid_char() {
        let _ = Suit::from('#');
    }
}
