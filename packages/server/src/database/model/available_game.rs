use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AvailableGameRow {
    pub id: Uuid,
    pub name: String,
    pub availability: String,
}
