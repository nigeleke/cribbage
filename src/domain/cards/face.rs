use super::rank::Rank;
use super::value::Value;

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

/// A Card face.
#[derive(Clone, Copy, Debug, Deserialize, Sequence, Serialize, PartialEq)]
pub enum Face {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Face {
    pub fn rank(&self) -> Rank {
        let rank = match self {
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
        Rank::from(rank)
    }

    pub fn value(&self) -> Value {
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
            Face::Ten | Face::Jack | Face::Queen | Face::King => 10,
        };
        Value::from(value)
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
        s.fmt(f)
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_face_from_valid_char() {
        for c in "A2345678TJK".as_bytes() {
            let _ = Face::from(*c as char);
        }
    }

    #[test]
    #[should_panic]
    fn cannot_create_face_from_invalid_char() {
        let _ = Face::from('#');
    }
}
