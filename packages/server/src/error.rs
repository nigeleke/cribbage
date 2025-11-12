use thiserror::*;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ServiceError(#[from] crate::services::ServiceError),

    #[error(transparent)]
    DatabaseError(#[from] crate::database::DatabaseError),

    #[error("mutex error: {0}")]
    MutexError(String),

    #[error(transparent)]
    StreamingError(#[from] tokio::sync::broadcast::error::RecvError),

    #[error(transparent)]
    StreamingMappingError(#[from] tokio_stream::wrappers::errors::BroadcastStreamRecvError),

    #[error("internal error: {0}")]
    InternalError(String),
}
