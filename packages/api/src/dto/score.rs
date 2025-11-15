use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreDTO {
    pub back_peg: usize,
    pub front_peg: usize,
}
