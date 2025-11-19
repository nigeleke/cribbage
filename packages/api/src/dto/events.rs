use serde::{Deserialize, Serialize};

use crate::GameIdDTO;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailableGameEventDTO {
    Created { game_id: GameIdDTO, name: String },
    Removed { game_id: GameIdDTO, name: String },
}
