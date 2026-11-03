#![cfg_attr(test, feature(coverage_attribute))]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod game;
mod lobby;
mod user;

#[cfg(test)]
mod macros;

#[cfg(test)]
#[coverage(off)]
mod test_support;

/// Commonly used types from the crate.
pub mod prelude {
    pub use super::game::{
        DeckSource, Game, GameCommand, GameError, GameEvent, GameId, GameState, Users,
    };
    pub use super::lobby::{Lobby, LobbyCommand, LobbyError, LobbyEvent, LobbyId, LobbyState};
    // pub use super::persistence::{PersistedCard, PersistedCards, PersistedDiscard};
    pub use super::user::UserId;
}
