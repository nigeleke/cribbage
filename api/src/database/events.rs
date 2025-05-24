use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TableChangeEvent<T> {
    pub operation: String,
    pub table: String,
    pub new_row: Option<T>,
    pub old_row: Option<T>,
}
