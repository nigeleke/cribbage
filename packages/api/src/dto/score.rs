use serde::{Deserialize, Serialize};

/// Represents a player's score for API clients.
///
/// The score is split into back and front pegs, following standard cribbage scoring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreDTO {
    /// Position of the back peg.
    pub back_peg: usize,

    /// Position of the front peg.
    pub front_peg: usize,
}

#[cfg(feature = "server")]
mod server_only {
    use server::domain::Position;

    use super::*;

    impl From<&Position> for ScoreDTO {
        fn from(value: &Position) -> Self {
            Self {
                back_peg: value.back().value(),
                front_peg: value.front().value(),
            }
        }
    }
}
