use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, types::Json};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
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
        id: Uuid,
        name: String,
        user_id1: Uuid,
        user_id2: Uuid,
        state: impl Into<Json<Value>>,
    ) -> Self {
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

impl From<ActiveGameRow> for ActiveGame {
    fn from(value: ActiveGameRow) -> Self {
        Self {
            id: GameId::from(value.id),
            name: value.name,
        }
    }
}

impl From<ActiveGame> for ActiveGameRow {
    fn from(value: ActiveGame) -> Self {
        Self {
            id: value.id.value(),
            name: value.name,
        }
    }
}
