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
        T: DeserializeOwned + Debug,
    {
        dioxus::prelude::info!("new_row_as 1 {:?}", self.new_row);
        let t = self
            .new_row
            .clone()
            .map(|r| serde_json::from_value::<T>(r))
            .transpose()?;
        dioxus::prelude::info!("new_row_as 2 {t:?}");
        Ok(t)
    }
}
