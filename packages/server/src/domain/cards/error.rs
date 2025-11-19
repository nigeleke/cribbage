use thiserror::*;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CardsError {
    #[error("invalid card")]
    InvalidCard(String),

    #[error("card {0} not found")]
    CardNotFound(String),

    #[error("cannot take cards required {0} of cards")]
    CardsNotAvailable(u8),
}
