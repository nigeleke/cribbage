use thiserror::*;

#[derive(Debug, Error)]
pub enum ProjectionError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    UuidError(#[from] uuid::Error),

    #[error(transparent)]
    DatabaseError(#[from] crate::database::DatabaseError),

    #[error(transparent)]
    ConversionError(#[from] crate::convertors::ConversionError),
}
