use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct StartedGameRow {
    pub unstarted_game_id: Uuid,
    pub active_game_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl StartedGameRow {
    pub fn new(unstarted_game_id: Uuid, active_game_id: Uuid) -> Self {
        let created_at = Utc::now();
        Self {
            unstarted_game_id,
            active_game_id,
            created_at,
        }
    }
}
