use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AvailableGameRow {
    pub id: Uuid,
    pub name: String,
    pub availability: String,
    pub created_at: DateTime<Utc>,
}

impl AvailableGameRow {
    pub fn new(id: Uuid, name: String, availability: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            name,
            availability,
            created_at,
        }
    }
}
