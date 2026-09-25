mod face;
mod rank;
mod suit;
mod value;

pub use face::Face;
pub use rank::Rank;
pub use suit::Suit;
pub use value::Value;

// ------------------------------------
/// A playing card consisting of a [`Face`] and a [`Suit`].
///
/// Note `Card` is `Copy`able.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    face: Face,
    suit: Suit,
}

impl Card {
    /// Constructs a new `Card` from a face and a suit.
    pub const fn new(face: Face, suit: Suit) -> Self {
        Self { face, suit }
    }

    /// Returns an iterator over **all** 52 standard playing cards.
    ///
    /// The cards are returned in the order of `Suit::iter()` × `Face::iter()`.
    pub fn all() -> Vec<Self> {
        use strum::IntoEnumIterator;
        let cards_for_suit = |s: Suit| Face::iter().map(move |f| Self::new(f, s));
        Suit::iter().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    /// Returns the face (rank) part of the card.
    pub fn face(&self) -> Face {
        self.face
    }

    /// Returns the suit part of the card.
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns the rank value used for most card games (Ace = 14, King = 13, …, Two = 2).
    ///
    /// See [`Face::rank()`] for details.
    pub fn rank(&self) -> Rank {
        self.face.rank()
    }

    /// Returns the numeric value of the card as used in a particular game.
    ///
    /// See [`Face::value()`] for details.
    pub fn value(&self) -> Value {
        self.face.value()
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.face, self.suit)
    }
}

pub fn cards_to_string(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|p| format!("{:?}", p))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
#[coverage(off)]
mod tests;
