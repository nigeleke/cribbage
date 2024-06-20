use super::rank::{Rank, HasRank};
use super::value::{Value, HasValue};

use enum_iterator::Sequence;
use serde::{Serialize, Deserialize};

/// A Card face.
#[derive(Clone, Copy, Debug, Deserialize, Sequence, Serialize, PartialEq)]
pub enum Face { Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King }

pub trait HasFace {
    fn face(&self) -> Face;
}

impl HasRank for Face {
    fn rank(&self) -> Rank {
        let rank = 
            match self {
                Face::Ace => 1,
                Face::Two => 2,
                Face::Three => 3,
                Face::Four => 4,
                Face::Five => 5,
                Face::Six => 6,
                Face::Seven => 7,
                Face::Eight => 8,
                Face::Nine => 9,
                Face::Ten => 10,
                Face::Jack => 11,
                Face::Queen => 12,
                Face::King => 13,
            };
        Rank::new(rank)
    }
}

impl HasValue for Face {
    fn value(&self) -> Value {
        let value = match self {
            Face::Ace => 1,
            Face::Two => 2,
            Face::Three => 3,
            Face::Four => 4,
            Face::Five => 5,
            Face::Six => 6,
            Face::Seven => 7,
            Face::Eight => 8,
            Face::Nine => 9,
            Face::Ten |
            Face::Jack |
            Face::Queen |
            Face::King => 10,
        };
        Value::new(value)
    }
}

impl std::fmt::Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
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
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
impl From<char> for Face {
    fn from(value: char) -> Self {
        match value {
            'A' => Face::Ace,
            '2' => Face::Two,
            '3' => Face::Three,
            '4' => Face::Four,
            '5' => Face::Five,
            '6' => Face::Six,
            '7' => Face::Seven,
            '8' => Face::Eight,
            '9' => Face::Nine,
            'T' => Face::Ten,
            'J' => Face::Jack,
            'Q' => Face::Queen,
            'K' => Face::King,
            _ => panic!("Unknown face"),
        }
    }
}
