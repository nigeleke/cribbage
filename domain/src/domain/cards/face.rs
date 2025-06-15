use super::{rank::Rank, value::Value};
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

/// A Card face.
#[derive(Clone, Copy, Debug, Sequence, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
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

impl TryFrom<char> for Face {
    type Error = String;

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
            _ => Err("unknown face".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_face_from_valid_char() {
        for c in b"A2345678TJK" {
            let _ = Face::try_from(*c as char).expect("valid try_from");
        }
    }

    #[test]
    fn cannot_create_face_from_invalid_char() {
        let _ = Face::try_from('#').expect_err("invalid try_from");
    }
}
