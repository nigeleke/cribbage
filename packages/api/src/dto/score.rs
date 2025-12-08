use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreDTO {
    pub back_peg: usize,
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
