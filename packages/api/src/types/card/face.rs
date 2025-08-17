use super::{
    rank::{HasRank, Rank},
    value::{HasValue, Value},
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

/// A Card face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, AsRefStr)]
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
    pub fn name(&self) -> String {
        self.as_ref().to_string()
    }

    pub fn is_jack(&self) -> bool {
        self == &Self::Jack
    }
}

pub trait HasFace {
    fn face(&self) -> Face;

    fn face_name(&self) -> String {
        self.face().name()
    }
}

impl HasRank for Face {
    fn rank(&self) -> Rank {
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
}

impl HasValue for Face {
    fn value(&self) -> Value {
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

#[cfg(test)]
#[coverage(off)]
pub mod test {
    use super::*;

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
                other => Err(format!("Invalid face: {other}")),
            }
        }
    }

    #[test]
    fn face_has_rank_value_display_string_and_name() {
        let faces = [
            (Face::Ace, 1, 1, "A", "Ace"),
            (Face::Two, 2, 2, "2", "Two"),
            (Face::Three, 3, 3, "3", "Three"),
            (Face::Four, 4, 4, "4", "Four"),
            (Face::Five, 5, 5, "5", "Five"),
            (Face::Six, 6, 6, "6", "Six"),
            (Face::Seven, 7, 7, "7", "Seven"),
            (Face::Eight, 8, 8, "8", "Eight"),
            (Face::Nine, 9, 9, "9", "Nine"),
            (Face::Ten, 10, 10, "T", "Ten"),
            (Face::Jack, 11, 10, "J", "Jack"),
            (Face::Queen, 12, 10, "Q", "Queen"),
            (Face::King, 13, 10, "K", "King"),
        ];

        for (face, rank, value, display_string, name) in faces {
            assert_eq!(face.rank(), Rank::from(rank));
            assert_eq!(face.value(), Value::from(value));
            assert_eq!(face.to_string(), display_string);
            assert_eq!(face.name(), name);
        }
    }
}
