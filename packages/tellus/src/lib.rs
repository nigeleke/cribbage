#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod game;
// mod game_gate;
// mod lobby;
mod user;

#[cfg(test)]
mod macros;

/// Commonly used types from the crate.
pub mod prelude {
    pub use super::game::{
        DeckSource, Game, GameCommand, GameError, GameEvent, GameId, GameState, PersistedCard,
        PersistedCards, PersistedDiscard, Users,
    };
    pub use super::user::UserId;
}
