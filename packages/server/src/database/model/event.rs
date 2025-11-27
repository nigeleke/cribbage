use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::JsonValue};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct EventRow {
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub sequence: u64,
    pub event_type: String,
    pub event_version: String,
    pub payload: JsonValue,
    pub metadata: JsonValue,
    pub timestamp: DateTime<Utc>,
}
