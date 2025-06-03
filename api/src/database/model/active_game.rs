use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, types::Json};
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize)]
pub struct ActiveGameRow {
    pub id: Uuid,
    pub name: String,
    pub user_id1: Uuid,
    pub user_id2: Uuid,
    pub state: Json<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ActiveGameRow {
    pub fn new(
        name: String,
        user_id1: Uuid,
        user_id2: Uuid,
        state: impl Into<Json<Value>>,
    ) -> Self {
        let id = Uuid::new_v4();
        let state = state.into();
        let created_at = Utc::now();
        let updated_at = created_at;

        Self {
            id,
            name,
            user_id1,
            user_id2,
            state,
            created_at,
            updated_at,
        }
    }
}
