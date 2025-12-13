use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Operation {
    Insert,
    Update,
    Delete,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Timing {
    Before,
    After,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub operation: Operation,
    schema: String,
    pub table_name: String,
    pub timing: Timing,
    pub primary_key: Vec<PrimaryKeyPart>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimaryKeyPart {
    pub column: String,
    pub value: JsonValue,
}
