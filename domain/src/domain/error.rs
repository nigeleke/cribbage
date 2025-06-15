use super::{Card, Player};
use thiserror::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GameError {
    #[error("internal error")]
    InternalError(String),

    #[error("incorrect number of players: {0} given, 2 required")]
    IncorrectNumberOfPlayers(usize),

    #[error("player {0} not in game")]
    PlayerNotInGame(Player),

    #[error("only two cards can be discarded to the crib")]
    TooManyDiscards,

    #[error("cannot start; cut for dealer is not decisive")]
    CannotStart,

    #[error("cannot redraw; cut for dealer was decisive")]
    CannotRedraw,

    #[error("player {0} is not a participant of the current game")]
    InvalidPlayer(Player),

    #[error("player does not own card {0}")]
    InvalidCard(Card),

    #[error("player does not own all cards")]
    InvalidCards,

    #[error("not this player's turn to play")]
    PlayOrPassNotPermittedByPlayer,

    #[error("cannot play the desired card")]
    CannotPlayCard,

    #[error("not this player's turn to pass")]
    CannotPass,
}
