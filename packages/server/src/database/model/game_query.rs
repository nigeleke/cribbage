use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::JsonValue};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct GameQueryRow {
    pub id: String,
    pub version: i64,
    pub payload: JsonValue,
}
