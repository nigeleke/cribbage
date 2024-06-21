use crate::types::*;

use enum_iterator::all;
use serde::{Deserialize, Serialize};

/// A playing card.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Card(Face, Suit);

impl Card {
    pub fn all() -> Vec<Card> { 
        let cards_for_suit = |s: Suit| all::<Face>().map(move |f| Card(f, s));
        all::<Suit>().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    pub fn face_name(&self) -> String { format!("{:?}", self.face()).trim_matches('"').into() }

    pub fn suit_name(&self) -> String { format!("{:?}", self.suit()).trim_matches('"').into() }
}

impl HasFace for Card {
    fn face(&self) -> Face {
        self.0
    }
}

impl HasSuit for Card {
    fn suit(&self) -> Suit {
        self.1
    }
}

impl HasRank for Card {
    fn rank(&self) -> Rank {
        self.face().rank()
    }
}

impl HasValue for Card {
    fn value(&self) -> Value {
        self.face().value()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face_suit = format!("{}{}", self.face(), self.suit());
        write!(f, "{}", face_suit)
    }
}

impl std::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.face() == other.face() &&
        self.suit() == other.suit()
    }
}

impl std::cmp::Eq for Card {}

#[cfg(test)]
impl From<&str> for Card {
    fn from(cid: &str) -> Self {
        let mut chars = cid.chars();
        Self(Face::from(chars.next().unwrap()), Suit::from(chars.next().unwrap()))
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
        let expected_face_name = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

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
