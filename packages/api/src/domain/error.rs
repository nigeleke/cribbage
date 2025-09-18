use thiserror::Error;

use crate::{Card, GameId, Player, UserId};

/// Represents errors that may occur during game or user operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GameError {
    /// The specified game is invalid or unrecognized.
    /// - `GameId`: The game that caused the error.
    #[error("invalid game {0}")]
    InvalidGame(GameId),

    /// The specified user is invalid or unrecognized.
    /// - `UserId`: The user that caused the error.
    #[error("invalid user {0}")]
    InvalidUser(UserId),

    /// The specified opponent is invalid in the given context.
    /// - `UserId`: The opponent that caused the error.
    #[error("invalid opponent {0}")]
    InvalidOpponent(UserId),

    /// The requested action is not permitted.
    /// - `String`: A description of the denied action.
    #[error("{0} is not permitted")]
    NotPermitted(String),

    /// The supplied "discards" cannot be discarded.
    /// Two discards are needed, both of which should be held
    /// by the discarding player.
    #[error("invalid discards: {0}")]
    InvalidDiscards(String),

    #[error("not the player's turn: {0}")]
    NotPlayersTurn(Player),

    #[error("invalid play: {0}")]
    InvalidPlay(Card),

    #[error("invalid pass: some cards are playable")]
    InvalidPass,
}
