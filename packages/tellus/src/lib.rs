#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
// #![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod game;
// mod game_gate;
// mod lobby;
mod persistence;
mod user;

#[cfg(test)]
mod macros;

/// This module provides a convenient set of imports for working with a tellus actor.
pub mod prelude {
    pub use super::game::{DeckSource, Game, GameCommand, GameEvent, GameId, GameState, Users};
    pub use super::persistence::{PersistedCard, PersistedCards};
    pub use super::user::UserId;
}
