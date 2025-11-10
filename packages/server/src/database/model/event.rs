use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::JsonValue;

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

impl EventRow {
    pub fn new(
        aggregate_type: String,
        aggregate_id: String,
        sequence: u64,
        event_type: String,
        event_version: String,
        payload: JsonValue,
        metadata: JsonValue,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            aggregate_type,
            aggregate_id,
            sequence,
            event_type,
            event_version,
            payload,
            metadata,
            timestamp,
        }
    }
}
