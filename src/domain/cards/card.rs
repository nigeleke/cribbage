use super::face::Face;
use super::rank::Rank;
use super::suit::Suit;
use super::value::Value;

use enum_iterator::all;
use serde::{Deserialize, Serialize};

/// A playing card.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Card {
    face: Face,
    suit: Suit,
}

impl Card {
    fn new(face: Face, suit: Suit) -> Self {
        Self { face, suit }
    }

    pub fn all() -> Vec<Card> {
        let cards_for_suit = |s: Suit| all::<Face>().map(move |f| Card::new(f, s));
        all::<Suit>().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    pub fn face_name(&self) -> String {
        format!("{:?}", self.face()).trim_matches('"').into()
    }

    pub fn suit_name(&self) -> String {
        format!("{:?}", self.suit()).trim_matches('"').into()
    }

    pub fn face(&self) -> Face {
        self.face
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

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face, self.suit)
    }
}

impl std::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.face() == other.face() && self.suit() == other.suit()
    }
}

impl std::cmp::Eq for Card {}

impl From<&str> for Card {
    fn from(cid: &str) -> Self {
        let mut chars = cid.chars();
        Self {
            face: Face::from(chars.next().unwrap()),
            suit: Suit::from(chars.next().unwrap()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cards_have_definitive_names() {
        let suits = "HCDS";
        let faces = "A23456789TJQK";

        let expected_suit_name = vec!["Hearts", "Clubs", "Diamonds", "Spades"];
        let expected_face_name = vec![
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        for (si, suit) in suits.chars().enumerate() {
            for (fi, face) in faces.chars().enumerate() {
                let cid = format!("{}{}", face, suit);
                let card = Card::from(cid.as_str());
                assert_eq!(card.suit_name(), expected_suit_name[si]);
                assert_eq!(card.face_name(), expected_face_name[fi])
            }
        }
    }
}
