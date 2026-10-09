use strum::{EnumIter, FromRepr};

/// The four suits in a standard French playing card deck.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter, FromRepr)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Suit {
    #[doc(hidden)] Hearts,
    #[doc(hidden)] Clubs,
    #[doc(hidden)] Diamonds,
    #[doc(hidden)] Spades,
}

impl std::fmt::Debug for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Suit::Hearts => "H",
            Suit::Clubs => "C",
            Suit::Diamonds => "D",
            Suit::Spades => "S",
        })
    }
}
