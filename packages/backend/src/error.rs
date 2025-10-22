use thiserror::Error;

use crate::GameId;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("{0}")]
    VarError(#[from] std::env::VarError),

    #[error("{0}")]
    DatabaseError(#[from] crate::database::DatabaseError),

    #[error("{0}")]
    SqlxError(#[from] sqlx::error::Error),

    #[error("{0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("{0}")]
    DomainError(#[from] crate::domain::DomainError),

    #[error("{0}")]
    StrumParseError(#[from] strum::ParseError),

    #[error("{0}")]
    JsonError(#[from] serde_json::Error),

    #[error("game not found: {0}")]
    GameNotFound(GameId),
}
