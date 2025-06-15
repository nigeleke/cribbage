use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

impl From<AvailableGameRow> for AvailableGame {
    fn from(value: AvailableGameRow) -> Self {
        if value.source == "Active" {
            Self::Active {
                id: GameId::from(value.id),
                name: value.name,
            }
        } else {
            Self::Lobby {
                id: GameId::from(value.id),
                name: value.name,
            }
        }
    }
}
