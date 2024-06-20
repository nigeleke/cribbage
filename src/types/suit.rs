use enum_iterator::Sequence;
use serde::{Serialize, Deserialize};

/// A Card suit.
#[derive(Clone, Copy, Debug, Deserialize, Sequence, Serialize, PartialEq)]
pub enum Suit { Hearts, Clubs, Diamonds, Spades }

pub trait HasSuit {
    fn suit(&self) -> Suit;
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Hearts => "♥",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Spades => "♠",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
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
