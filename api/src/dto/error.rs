use thiserror::*;

#[derive(Debug, Error)]
pub enum DtoError {
    #[error("Cannot deserialise {0}")]
    CannotDeserialise(#[from] serde_json::Error),

    #[error("Game error {0}")]
    GameError(String),
}

#[cfg(feature = "server")]
impl From<domain::GameError> for DtoError {
    fn from(err: domain::GameError) -> Self {
        Self::GameError(err.to_string())
    }
}
