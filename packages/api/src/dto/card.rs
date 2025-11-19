use serde::{Deserialize, Serialize};

pub type CardIdDTO = String;

/// A DTO representing a Card. The value is the "cid" (Card identifier), e.g. "AS", "QD".
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardDTO {
    FaceUp { cid: CardIdDTO },
    FaceDown,
}

#[cfg(feature = "server")]
mod server_only {
    use super::*;
    use server::domain::Card;

    impl CardDTO {
        pub fn face_up(card: &Card) -> Self {
            Self::FaceUp {
                cid: card.cid().clone(),
            }
        }

        pub fn face_down(_card: &Card) -> Self {
            Self::FaceDown
        }
    }
}
