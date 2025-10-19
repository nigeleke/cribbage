use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LobbyGameRow {
    pub id: Uuid,
    pub host_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl LobbyGameRow {
    pub fn new(id: Uuid, host_id: Uuid, name: String) -> Self {
        let created_at = Utc::now();

        Self {
            id,
            host_id,
            name,
            created_at,
        }
    }
}
