use crate::user::UserId;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum GameError {
    #[error("game-error.unknown-user")]
    UnknownUser(UserId),

    #[error("game-error.invalid-deck")]
    InvalidDeck,

    #[error("game-error.invalid-card")]
    InvalidCard,

    #[error("game-error.domain-error")]
    DomainError(#[from] cribbage_domain::prelude::GameError),
}
