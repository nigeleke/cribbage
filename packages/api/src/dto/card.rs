use serde::{Deserialize, Serialize};

/// A string identifier for a card.
///
/// Examples include `"AS"` for Ace of Spades or `"QD"` for Queen of Diamonds.
pub type CardIdDTO = String;

/// A DTO representing a Card. The value is the "cid" (Card identifier), e.g. "AS", "QD".
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardDTO {
    /// The card is visible; contains the card identifier (`cid`), e.g., `"AS"`.
    FaceUp {
        /// The card id as a two character string format.
        cid: CardIdDTO,
    },

    /// The card is hidden and its identity is unknown.
    FaceDown,
}

#[cfg(feature = "server")]
mod server_only {
    use server::domain::Card;

    use super::*;

    impl CardDTO {
        /// Constructs a face-up DTO from a domain `Card`.
        pub fn face_up(card: &Card) -> Self {
            Self::FaceUp {
                cid: CardIdDTO::from(card.cid()),
            }
        }

        /// Constructs a face-down DTO from a domain `Card`.
        /// The card itself isn't required during the construction.
        pub fn face_down(_card: &Card) -> Self {
            Self::FaceDown
        }
    }
}
