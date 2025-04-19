use enum_iterator::all;

use super::{face::Face, rank::Rank, suit::Suit, value::Value};

/// A playing card.
#[derive(Clone, Copy, Debug)]
pub struct Card {
    face: Face,
    suit: Suit,
}

impl Card {
    const fn new(face: Face, suit: Suit) -> Self {
        Self { face, suit }
    }

    pub fn all() -> Vec<Self> {
        let cards_for_suit = |s: Suit| all::<Face>().map(move |f| Self::new(f, s));
        all::<Suit>().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    pub fn face_name(&self) -> String {
        format!("{:?}", self.face()).trim_matches('"').into()
    }

    pub fn suit_name(&self) -> String {
        format!("{:?}", self.suit()).trim_matches('"').into()
    }

    pub const fn face(&self) -> Face {
        self.face
    }

    pub const fn suit(&self) -> Suit {
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

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();

        let face = chars
            .next()
            .ok_or_else(|| "char required for card face".to_string())?;
        let face = Face::try_from(face)?;

        let suit = chars
            .next()
            .ok_or_else(|| "char required for card suit".to_string())?;
        let suit = Suit::try_from(suit)?;

        Ok(Self { face, suit })
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
                let card = Card::try_from(cid.as_str()).expect("valid cards str");
                assert_eq!(card.suit_name(), expected_suit_name[si]);
                assert_eq!(card.face_name(), expected_face_name[fi])
            }
        }
    }

    #[test]
    fn cannot_create_from_invalid_face() {
        let error = Card::try_from("#S").expect_err("invalid face");
        assert_eq!(error, "unknown face");
    }

    #[test]
    fn cannot_create_from_invalid_suit() {
        let error = Card::try_from("A#").expect_err("invalid suit");
        assert_eq!(error, "unknown suit");
    }

    #[test]
    fn cannot_create_from_empty_str() {
        let error = Card::try_from("").expect_err("invalid card");
        assert_eq!(error, "char required for card face");
    }

    #[test]
    fn cannot_create_from_short_str() {
        let error = Card::try_from("A").expect_err("invalid card");
        assert_eq!(error, "char required for card suit");
    }
}
