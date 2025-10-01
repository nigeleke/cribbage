use chrono::{DateTime, Utc};
use dto::{AvailableGameDTO, GameIdDTO};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::server::database::DatabaseError;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AvailableGameRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub source: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl AvailableGameRow {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        source: String,
        name: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            source,
            name,
            created_at,
        }
    }
}

impl TryFrom<AvailableGameRow> for AvailableGameDTO {
    type Error = DatabaseError;

    fn try_from(value: AvailableGameRow) -> Result<Self, Self::Error> {
        match value.source.as_str() {
            "Lobby" => Ok(AvailableGameDTO::Lobby {
                game_id: GameIdDTO::new(value.id),
                name: value.name,
            }),
            "Active" => Ok(AvailableGameDTO::Active {
                game_id: GameIdDTO::new(value.id),
                name: value.name,
            }),
            _ => Err(DatabaseError::InvalidValue(
                "available_game::source".into(),
                value.source,
            )),
        }
    }
}
