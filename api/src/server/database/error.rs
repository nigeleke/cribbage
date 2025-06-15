use thiserror::*;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("invalid environment setting: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("database access failed: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("database migration failed: {0}")]
    SqlxMigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("already initialized")]
    AlreadyInitialized,

    #[error("database not initialized correctly")]
    Uninitialized,

    #[error("serde_json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("invalid operation: {0} {1}")]
    InvalidOperation(String, String),

    #[error("missing data: {0}")]
    MissingData(String),
}
