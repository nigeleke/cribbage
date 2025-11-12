use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreDTO {
    back_peg: usize,
    front_peg: usize,
}

impl ScoreDTO {
    pub fn new(back_peg: usize, front_peg: usize) -> Self {
        Self {
            back_peg,
            front_peg,
        }
    }
}
