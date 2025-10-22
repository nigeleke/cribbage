use serde::{Deserialize, Serialize};

/// A DTO representing a Card. The value is the "cid" (Card identifier), e.g. "AS", "QD".
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardDTO {
    FaceUp { cid: String },
    FaceDown,
}

// impl std::fmt::Display for CardDTO {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }
