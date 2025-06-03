use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, types::Json};
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize)]
pub struct UserGameRow {
    pub game_id: Uuid,
    pub user_id: Uuid,
    pub state: Json<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
