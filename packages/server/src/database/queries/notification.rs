use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

use crate::database::DatabaseError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub operation: String,
    pub table_name: String,
    pub timing: String,
    pub new_row: Option<JsonValue>,
    pub old_row: Option<JsonValue>,
}

impl Notification {
    pub fn new_row_as<T>(&self) -> Result<Option<T>, DatabaseError>
    where
        T: DeserializeOwned,
    {
        let t = self
            .new_row
            .clone()
            .map(|r| serde_json::from_value::<T>(r))
            .transpose()?;
        Ok(t)
    }

    pub fn old_row_as<T>(&self) -> Result<Option<T>, DatabaseError>
    where
        T: DeserializeOwned,
    {
        let t = self
            .old_row
            .clone()
            .map(|r| serde_json::from_value::<T>(r))
            .transpose()?;
        Ok(t)
    }
}
