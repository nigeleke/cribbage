use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AvailableGameRow {
    pub id: Uuid,
    pub name: String,
    pub availability: String,
}

impl AvailableGameRow {
    pub fn new(id: Uuid, name: String, availability: String) -> Self {
        Self {
            id,
            name,
            availability,
        }
    }
}
