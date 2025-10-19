use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    operation: String,
    table_name: String,
    timing: String,
    new_row: String,
    old_row: String,
}
