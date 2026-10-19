use crate::user::UserId;

/// Errors that can occur while processing a cribbage game.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum GameError {
    /// The user is not participating in the game.
    #[error("game-error.unknown-user")]
    UnknownUser(UserId),

    /// Persisted game data could not be decoded.
    #[error("game-error.decode-error")]
    DecodeError,

    /// The event cannot be applied in the current game state.
    #[error("game-error.unhandled-event")]
    UnhandledEvent,

    /// An error reported by the cribbage domain.
    #[error("game-error.{0}")]
    DomainError(#[from] cribbage_domain::prelude::DomainError),
}
