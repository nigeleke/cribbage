use thiserror::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot deserialise {0}")]
    CannotDeserialise(#[from] serde_json::Error),

    #[error("Game error {0}")]
    GameError(String),
}

#[cfg(feature = "server")]
impl From<domain::GameError> for Error {
    fn from(err: domain::GameError) -> Self {
        Error::GameError(err.to_string())
    }
}
