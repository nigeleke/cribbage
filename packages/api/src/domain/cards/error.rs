use thiserror::*;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CardsError {
    #[error("card {0} not found")]
    CardNotFound(String),

    #[error("not enough cards available to take")]
    CardsNotAvailable,
}
