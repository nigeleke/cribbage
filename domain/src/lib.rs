#![feature(coverage_attribute)]

mod constants;
mod display;
mod domain;

#[cfg(test)]
mod test_modules;

pub use constants::NUMBER_OF_PLAYERS_IN_GAME;
pub use domain::{
    Card, Crib, Cut, Deck, Game, GameError, Hand, HasCrib, HasCut, HasDeck, HasHands, HasPlayers,
    HasScores, HasState, Pegging, Play, PlayState, Player, Players, State,
};
