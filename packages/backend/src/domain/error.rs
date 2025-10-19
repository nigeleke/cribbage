use thiserror::Error;

use crate::domain::{Card, GameId, Player, UserId};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("invalid game {0}")]
    InvalidGame(GameId),

    #[error("invalid user {0}")]
    InvalidUser(UserId),

    #[error("invalid opponent {0}")]
    InvalidOpponent(UserId),

    #[error("{0} is not permitted")]
    NotPermitted(String),

    #[error("invalid discards: {0}")]
    InvalidDiscards(String),

    #[error("not the player's turn: {0}")]
    NotPlayersTurn(Player),

    #[error("invalid play: {0}")]
    InvalidPlay(Card),

    #[error("invalid pass: some cards are playable")]
    InvalidPass,
}
