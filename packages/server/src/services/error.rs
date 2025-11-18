use thiserror::*;

use crate::domain::{GameId, UserId};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    DatabaseError(#[from] crate::database::DatabaseError),

    #[error(transparent)]
    ConversionError(#[from] crate::convertors::ConversionError),

    #[error(transparent)]
    PersistenceError(#[from] cqrs_es::persist::PersistenceError),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ParseError(#[from] strum::ParseError),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error("game not found: {0}")]
    GameNotFound(GameId),

    #[error("invalid user: {0}")]
    InvalidUser(UserId),

    #[error(transparent)]
    GameError(#[from] crate::domain::GameError),

    #[error(transparent)]
    AggregateError(#[from] cqrs_es::AggregateError<crate::domain::GameError>),

    #[error(transparent)]
    BroadcastStreamError(#[from] tokio_stream::wrappers::errors::BroadcastStreamRecvError),
}
