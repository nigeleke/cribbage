use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewGame {
    pub name: String,
    pub host_id: Uuid,
    pub guest_id: Option<Uuid>,
    pub state: Json<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGame {
    pub id: Uuid,
    pub name: Option<String>,
    pub guest_id: Option<Uuid>,
    pub state: Option<Json<Value>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct GameRow {
    pub id: Uuid,
    pub name: String,
    pub host_id: Uuid,
    pub guest_id: Option<Uuid>,
    pub state: Json<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
