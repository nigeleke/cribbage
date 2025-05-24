#[cfg(feature = "server")]
use thiserror::*;

#[cfg(feature = "server")]
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::database::DatabaseError),

    #[error("Unable to deserialise {0}")]
    JsonError(String),

    #[error("Invalid table {0}")]
    InvalidTable(String),

    #[error("Missing field {0}")]
    MissingField(String),

    #[error("Invalid operation {0}")]
    InvalidOperation(String),
}
