use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sqlx::types::JsonValue;

use crate::{bug, error::ServerError};

type Result<T> = std::result::Result<T, ServerError>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Change<T> {
    Insert { t: T },
    Update { old_t: T, new_t: T },
    Delete { t: T },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Operation {
    INSERT,
    UPDATE,
    DELETE,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Timing {
    BEFORE,
    AFTER,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    operation: Operation,
    pub table_name: String,
    timing: Timing,
    pub new_row: Option<JsonValue>,
    pub old_row: Option<JsonValue>,
}

impl Notification {
    pub fn as_change<T>(&self) -> Result<Change<T>>
    where
        T: DeserializeOwned,
    {
        let missing = |s: &str| bug!()(format!("{s} notification data missing"));
        let old_row = || self.old_row_as::<T>()?.ok_or_else(|| missing("old_row"));
        let new_row = || self.new_row_as::<T>()?.ok_or_else(|| missing("new_row"));

        let change = match self.operation {
            Operation::INSERT => Change::Insert { t: new_row()? },
            Operation::UPDATE => Change::Update {
                old_t: old_row()?,
                new_t: new_row()?,
            },
            Operation::DELETE => Change::Delete { t: old_row()? },
        };

        Ok(change)
    }

    fn new_row_as<T>(&self) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        self.new_row
            .clone()
            .map(|r| serde_json::from_value::<T>(r))
            .transpose()
            .map_err(bug!())
    }

    fn old_row_as<T>(&self) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        self.old_row
            .clone()
            .map(|r| serde_json::from_value::<T>(r))
            .transpose()
            .map_err(bug!())
    }
}
