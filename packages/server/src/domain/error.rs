use thiserror::Error;

use crate::domain::{Card, Player};

/// Domain-specific errors that represent invalid game actions.
///
/// These errors are returned when a player attempts an illegal move according
/// to the rules of the game (e.g. playing out of turn, invalid discard, etc.).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    /// The host and guest must be different users.
    #[error("invalid opponent")]
    InvalidOpponent,

    /// The user is not permitted to perform the action, i.e. they are not
    /// a player in the game, or they are trying a command in the inccorect
    /// phase in the game.
    #[error("{0} is not permitted")]
    NotPermitted(String),

    /// The user is discarding cards they do not own, or too many or too few cards.
    #[error("invalid discards: {0}")]
    InvalidDiscards(String),

    /// The user is making a `Play` or `Go` when it is not their turn.
    #[error("not the player's turn: {0}")]
    NotPlayersTurn(Player),

    /// The user is playing card they do not own, or that will make the
    /// running total > 31.
    #[error("invalid play: {0}")]
    InvalidPlay(Card),

    /// The user is declaring `Go` even though they have a valid card that
    /// must be played.
    #[error("invalid go: some cards are playable")]
    InvalidGo,
}
