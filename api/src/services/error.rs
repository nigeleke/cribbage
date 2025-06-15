#[cfg(feature = "server")]
use thiserror::*;

#[cfg(feature = "server")]
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Redis pool error: {0}")]
    DeadpoolRedis(#[from] deadpool_redis::PoolError),

    #[error("Redis error: {0}")]
    Redis(#[from] deadpool_redis::redis::RedisError),

    #[error("Database error: {0}")]
    Database(#[from] crate::database::DatabaseError),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid table {0}")]
    InvalidTable(String),

    #[error("Missing field {0}")]
    MissingField(String),

    #[error("Invalid operation {0}")]
    UnexpectedOperation(String),

    #[error("Game error: {0}")]
    GameErr(#[from] domain::GameError),

    #[error("Invalid state for action: {0}")]
    InvalidState(String),

    #[error("Transmission object error: {0}")]
    DtoError(#[from] crate::dto::DtoError),
}
