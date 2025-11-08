use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::JsonValue;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGame {
    pub id: Uuid,
    pub name: String,
    pub host_id: Uuid,
    pub guest_id: Option<Uuid>,
    pub state: JsonValue,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpdateGame {
    pub id: Uuid,
    pub name: Option<String>,
    pub host_id: Option<Uuid>,
    pub guest_id: Option<Uuid>,
    pub state: Option<JsonValue>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct GameRow {
    pub id: Uuid,
    pub name: String,
    pub host_id: Uuid,
    pub guest_id: Option<Uuid>,
    pub state: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
