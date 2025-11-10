use thiserror::*;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    SqlxMigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("already initialized")]
    AlreadyInitialized,

    #[error("database not initialized correctly")]
    Uninitialized,

    #[error("invalid operation: {0} {1}")]
    InvalidOperation(String, String),

    #[error("missing data: {0}")]
    MissingData(String),

    #[error("invalid value: {0} {1}")]
    InvalidValue(String, String),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}
