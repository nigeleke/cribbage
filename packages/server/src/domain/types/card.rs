use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::domain::CardsError;
mod face;
mod rank;
mod suit;
mod value;

pub use self::{face::Face, rank::Rank, suit::Suit, value::Value};

/// A playing card consisting of a [`Face`] and a [`Suit`].
///
/// Note `Card` is `Copy`able.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    face: Face,
    suit: Suit,
}

impl Card {
    /// Constructs a new `Card` from a face and a suit.
    #[must_use]
    const fn new(face: Face, suit: Suit) -> Self {
        Self { face, suit }
    }

    /// Returns an iterator over **all** 52 standard playing cards.
    ///
    /// The cards are returned in the order of `Suit::iter()` × `Face::iter()`.
    #[must_use]
    pub fn all() -> Vec<Self> {
        let cards_for_suit = |s: Suit| Face::iter().map(move |f| Self::new(f, s));
        Suit::iter().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    /// Returns a compact identifier string for the card (e.g. `"AS"`, `"KD"`, `"T♥"`).
    ///
    /// The format is single character anglicised face abbreviation followed by the
    /// single character anglicised suit abbreviation,
    #[must_use]
    pub fn cid(&self) -> String {
        let Self { face, suit } = self;
        format!("{face}{suit}")
    }

    /// Returns the face (rank) part of the card.
    #[must_use]
    pub fn face(&self) -> Face {
        self.face
    }

    /// Helper function returning `true` if this card is a Jack, regardless of suit.
    #[must_use]
    pub fn is_jack(&self) -> bool {
        self.face.is_jack()
    }

    /// Returns the suit part of the card.
    #[must_use]
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns the rank value used for most card games (Ace = 14, King = 13, …, Two = 2).
    ///
    /// See [`Face::rank()`] for details.
    #[must_use]
    pub fn rank(&self) -> Rank {
        self.face.rank()
    }

    /// Returns the numeric value of the card as used in a particular game.
    ///
    /// See [`Face::value()`] for details.
    #[must_use]
    pub fn value(&self) -> Value {
        self.face.value()
    }
}

impl std::str::FromStr for Card {
    type Err = CardsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 2 {
            let mut chars = s.chars();
            let face = chars.next().ok_or(CardsError::InvalidCard(s.into()))?;
            let face = Face::try_from(face)?;
            let suit = chars.next().ok_or(CardsError::InvalidCard(s.into()))?;
            let suit = Suit::try_from(suit)?;
            Ok(Card::new(face, suit))
        } else {
            Err(CardsError::InvalidCard(s.into()))
        }
    }
}

#[cfg(test)]
pub(crate) trait CardExt {
    fn cards_from(self) -> Result<Vec<Card>, CardsError>;
}

#[cfg(test)]
impl CardExt for &str {
    fn cards_from(self) -> Result<Vec<Card>, CardsError> {
        use std::str::FromStr;

        if !self.len().is_multiple_of(2) {
            Err(CardsError::InvalidCard(self.into()))
        } else {
            (0..self.len())
                .step_by(2)
                .map(|i| Card::from_str(&self[i..i + 2]))
                .collect()
        }
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
    use super::*;

    #[test]
    fn cards_have_definitive_id_rank_and_value() {
        use std::str::FromStr;

        let suits = "HCDS";
        let faces = "A23456789TJQK";

        for suit in suits.chars() {
            for face in faces.chars() {
                let cid = format!("{face}{suit}");
                let face = Face::try_from(face).expect("valid face");
                let card = Card::from_str(cid.as_str()).expect("valid cards str");
                assert_eq!(card.cid(), cid);
                assert_eq!(card.rank(), face.rank());
                assert_eq!(card.value(), face.value());
            }
        }
    }
}
