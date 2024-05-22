use super::player::Player;
use super::card::Card;

#[derive(Debug, PartialEq)]
pub enum Error {
    TooManyPlayers,
    NotEnoughPlayers,
    CutForStartUndecided,
    CutForStartDecided,
    InvalidAction(String),
    InvalidPlayer(Player),
    InvalidCard(Card),
    TooManyDiscards,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "Internal error: TODO: Expand on this"
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
