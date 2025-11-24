use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::dto::GameIdDTO;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailabilityDTO {
    Private,
    Public,
}

/// Games available for the end user to play.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableGameDTO {
    /// Game identifier sortable by creation time.
    pub game_id: GameIdDTO,

    /// A human-readable name for the game, such as "liberated-happy-donkey".
    pub name: String,

    pub availability: AvailabilityDTO,

    pub created_at: DateTime<Utc>,
}
