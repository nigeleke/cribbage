use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database::DatabaseError;

#[derive(Debug, Serialize, Deserialize)]
struct PgNotify {
    operation: String,
    table_name: String,
    timing: String,
    old_row: Option<Value>,
    new_row: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TableChangeEvent<T: std::fmt::Debug> {
    InsertBefore {
        table_name: String,
        new_row: T,
    },
    InsertAfter {
        table_name: String,
        new_row: T,
    },
    UpdateBefore {
        table_name: String,
        new_row: T,
        old_row: T,
    },
    UpdateAfter {
        table_name: String,
        new_row: T,
        old_row: T,
    },
    DeleteBefore {
        table_name: String,
        old_row: T,
    },
    DeleteAfter {
        table_name: String,
        old_row: T,
    },
}

impl<T: std::fmt::Debug + DeserializeOwned> TryFrom<PgNotify> for TableChangeEvent<T> {
    type Error = DatabaseError;

    fn try_from(value: PgNotify) -> Result<Self, Self::Error> {
        let event = match (value.operation.as_str(), value.timing.as_str()) {
            ("INSERT", "BEFORE") => Self::InsertBefore {
                table_name: value.table_name,
                new_row: value
                    .new_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("new_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
            },
            ("INSERT", "AFTER") => Self::InsertAfter {
                table_name: value.table_name,
                new_row: value
                    .new_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("new_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
            },
            ("UPDATE", "BEFORE") => Self::UpdateBefore {
                table_name: value.table_name,
                new_row: value
                    .new_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("new_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
                old_row: value
                    .old_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("old_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
            },
            ("UPDATE", "AFTER") => Self::UpdateAfter {
                table_name: value.table_name,
                new_row: value
                    .new_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("new_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
                old_row: value
                    .old_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("old_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
            },
            ("DELETE", "BEFORE") => Self::DeleteBefore {
                table_name: value.table_name,
                old_row: value
                    .old_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("old_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
            },
            ("DELETE", "AFTER") => Self::DeleteAfter {
                table_name: value.table_name,
                old_row: value
                    .old_row
                    .map(serde_json::from_value::<T>)
                    .ok_or_else(|| DatabaseError::MissingData("old_row".to_string()))?
                    .map_err(DatabaseError::SerdeJsonError)?,
            },
            _ => {
                return Err(DatabaseError::InvalidOperation(
                    value.operation,
                    value.timing,
                ));
            }
        };

        Ok(event)
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for TableChangeEvent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

// pub async fn listen_table_changes<T: DeserializeOwned + std::fmt::Debug>(
//     table_name: &str,
//     pool: &PgPool,
// ) -> Result<impl Stream<Item = Result<TableChangeEvent<T>, DatabaseError>>, DatabaseError> {
//     let mut listener = PgListener::connect_with(pool).await?;
//     listener.listen(table_name).await?;

//     let stream = stream! {
//         loop {
//             match listener.try_recv().await {
//                 Ok(Some(notification)) => {
//                     println!("listen_table_changes: {:?}", notification);
//                     match serde_json::from_str::<PgNotify>(notification.payload()) {
//                         Ok(change) => yield TableChangeEvent::try_from(change),
//                         Err(e) => {
//                             warn!("Failed to deserialize {}: {}", table_name, e);
//                             yield Err(DatabaseError::SerdeJsonError(e))
//                         }
//                     }
//                 }
//                 Ok(None) => break,
//                 Err(e) => yield Err(DatabaseError::SqlxError(e))
//             }
//         }
//     };

//     Ok(stream)
// }
