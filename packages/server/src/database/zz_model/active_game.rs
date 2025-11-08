use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ActiveGameRow {
    pub id: Uuid,
    pub name: String,
    pub host_id: Uuid,
    pub guest_id: Uuid,
    pub state: Json<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ActiveGameRow {
    pub fn new(
        id: Uuid,
        name: String,
        host_id: Uuid,
        guest_id: Uuid,
        state: impl Into<Json<Value>>,
    ) -> Self {
        let state = state.into();
        let created_at = Utc::now();
        let updated_at = created_at;

        Self {
            id,
            name,
            host_id,
            guest_id,
            state,
            created_at,
            updated_at,
        }
    }
}
