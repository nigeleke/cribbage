use thiserror::*;

#[derive(Debug, Error)]
pub enum DtoError {
    #[error("Serde json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Uuid error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Game error {0}")]
    GameError(String),
}

#[cfg(feature = "server")]
impl From<domain::GameError> for DtoError {
    fn from(err: domain::GameError) -> Self {
        Self::GameError(err.to_string())
    }
}
