use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

mod face;
mod rank;
mod suit;
mod value;

pub use self::face::Face;
pub use self::rank::Rank;
pub use self::suit::Suit;
pub use self::value::Value;

/// A playing card.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    face: Face,
    suit: Suit,
}

impl Card {
    const fn new(face: Face, suit: Suit) -> Self {
        Self { face, suit }
    }

    pub const fn placeholder() -> Self {
        let face = Face::Ace;
        let suit = Suit::Spades;
        Self::new(face, suit)
    }

    pub fn all() -> Vec<Self> {
        let cards_for_suit = |s: Suit| Face::iter().map(move |f| Self::new(f, s));
        Suit::iter().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    pub fn cid(&self) -> String {
        let Self { face, suit } = self;
        format!("{face}{suit}")
    }

    pub fn name(&self) -> String {
        let Self { face, suit } = self;
        format!("{} of {}", face.name(), suit.name())
    }

    pub fn face(&self) -> Face {
        self.face
    }

    pub fn is_jack(&self) -> bool {
        self.face.is_jack()
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> Rank {
        self.face.rank()
    }

    pub fn value(&self) -> Value {
        self.face.value()
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card({})", self)
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cid().fmt(f)
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use std::str::FromStr;

    use super::*;

    impl FromStr for Card {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.len() == 2 {
                let mut chars = s.chars();
                let face = chars.next().ok_or("missing face charactor")?.try_into()?;
                let suit = chars.next().ok_or("missing suit character")?.try_into()?;
                Ok(Card::new(face, suit))
            } else {
                Err("two characters required for card".into())
            }
        }
    }

    #[test]
    fn cards_have_definitive_names_id_rank_and_value() {
        let suits = "HCDS";
        let faces = "A23456789TJQK";

        let expected_suit_name = ["Hearts", "Clubs", "Diamonds", "Spades"];
        let expected_face_name = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        for (si, suit) in suits.chars().enumerate() {
            for (fi, face) in faces.chars().enumerate() {
                let cid = format!("{face}{suit}");
                let face = Face::try_from(face).expect("valid face");
                let card = Card::from_str(cid.as_str()).expect("valid cards str");
                assert_eq!(
                    card.name(),
                    format!("{} of {}", expected_face_name[fi], expected_suit_name[si])
                );
                assert_eq!(card.cid(), cid);
                assert_eq!(card.rank(), face.rank());
                assert_eq!(card.value(), face.value());
            }
        }
    }
}
