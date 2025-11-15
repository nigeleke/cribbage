use super::{CardDTO, ScoreDTO};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerStateDTO {
    pub hand: Vec<CardDTO>,
    pub score: ScoreDTO,
}
