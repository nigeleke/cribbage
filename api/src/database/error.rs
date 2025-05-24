use thiserror::*;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Invalid environment setting: {0}")]
    InvalidEnvironmentSetting(#[from] std::env::VarError),

    #[error("Database access failed: {0}")]
    DatabaseAccessFailed(#[from] sqlx::Error),

    #[error("Problem found while migrating database: {0}")]
    MigrationFailed(#[from] sqlx::migrate::MigrateError),

    #[error("Already initialized")]
    AlreadyInitialized,

    #[error("Database not initialized correctly")]
    Uninitialized,

    #[error("Serialization failed: {0}")]
    CannotSerialize(#[from] serde_json::Error),
}
