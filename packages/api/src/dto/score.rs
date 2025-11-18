use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreDTO {
    pub back_peg: usize,
    pub front_peg: usize,
}

#[cfg(feature = "server")]
mod server_only {
    use super::*;
    use server::domain::Pegging;

    impl From<&Pegging> for ScoreDTO {
        fn from(value: &Pegging) -> Self {
            Self {
                back_peg: value.back_peg().value(),
                front_peg: value.front_peg().value(),
            }
        }
    }
}
