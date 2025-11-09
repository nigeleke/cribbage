use thiserror::*;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    DatabaseError(#[from] crate::database::DatabaseError),

    #[error(transparent)]
    GameError(#[from] crate::domain::GameError),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error("mutex error: {0}")]
    MutexError(String),

    #[error(transparent)]
    ParseError(#[from] strum::ParseError),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error(transparent)]
    AggregateError(#[from] cqrs_es::AggregateError<crate::GameError>),

    #[error(transparent)]
    PersistenceError(#[from] cqrs_es::persist::PersistenceError),
}
