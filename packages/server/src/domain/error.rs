use thiserror::Error;

use crate::domain::{Card, Player};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("invalid user")]
    InvalidUser,

    #[error("invalid opponent")]
    InvalidOpponent,

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
