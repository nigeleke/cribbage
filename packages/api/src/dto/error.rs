use thiserror::*;

/// Errors exposed by the API.
#[derive(Debug, Error)]
pub enum DTOError {
    /// String cannot be decoded into a Uuid.
    #[error("invalid id: {0}")]
    InvalidId(#[from] uuid::Error),
}
