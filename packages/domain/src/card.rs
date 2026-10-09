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
    pub(crate) fn all() -> Vec<Self> {
        use strum::IntoEnumIterator;
        let cards_for_suit = |s: Suit| Face::iter().map(move |f| Self::new(f, s));
        Suit::iter().flat_map(cards_for_suit).collect::<Vec<_>>()
    }

    /// Compare cards by their rank.
    pub(crate) fn cmp_rank(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }

    /// Returns the face part of the card.
    pub const fn face(&self) -> Face {
        self.face
    }

    /// Returns the suit part of the card.
    pub const fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns the rank value used for most card games (Ace = 14, King = 13, …, Two = 2).
    ///
    /// See [`Face::rank()`] for details.
    pub(crate) const fn rank(&self) -> Rank {
        self.face.rank()
    }

    /// Returns the numeric value of the card as used in a particular game.
    ///
    /// See [`Face::value()`] for details.
    pub(crate) const fn value(&self) -> Value {
        self.face.value()
    }
}

#[cfg(test)]
impl std::str::FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        (bytes.len() == 2)
            .then_some(())
            .ok_or("invalid card: two bytes required")?;

        let face = match bytes[0] {
            b'A' => Ok(Face::Ace),
            b'2' => Ok(Face::Two),
            b'3' => Ok(Face::Three),
            b'4' => Ok(Face::Four),
            b'5' => Ok(Face::Five),
            b'6' => Ok(Face::Six),
            b'7' => Ok(Face::Seven),
            b'8' => Ok(Face::Eight),
            b'9' => Ok(Face::Nine),
            b'T' => Ok(Face::Ten),
            b'J' => Ok(Face::Jack),
            b'Q' => Ok(Face::Queen),
            b'K' => Ok(Face::King),
            _ => Err("invalid card face"),
        }?;

        let suit = match bytes[1] {
            b'H' => Ok(Suit::Hearts),
            b'C' => Ok(Suit::Clubs),
            b'D' => Ok(Suit::Diamonds),
            b'S' => Ok(Suit::Spades),
            _ => Err("invalid card suit"),
        }?;

        Ok(Self::new(face, suit))
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.face, self.suit)
    }
}

pub(crate) fn cards_to_string(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|p| format!("{:?}", p))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
#[coverage(off)]
mod tests;
