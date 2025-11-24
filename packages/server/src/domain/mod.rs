mod available_game;
mod cards;
mod commands;
mod error;
mod events;
mod game;
mod players;
mod plays;
mod scoreboard;
mod scoring;
mod state;
mod types;

pub use available_game::*;
pub use cards::*;
pub use commands::*;
pub use error::*;
pub use events::*;
pub use game::*;
pub use players::*;
pub use plays::*;
pub use scoreboard::*;
pub use scoring::*;
pub use state::*;
pub use types::*;

pub(crate) mod constants {
    pub const PLAYER_COUNT: usize = 2;

    pub const CARDS_DEALT_PER_HAND: usize = 6;
    pub const CARDS_KEPT_PER_HAND: usize = 4;
    pub const CARDS_DISCARDED_TO_CRIB: usize = CARDS_DEALT_PER_HAND - CARDS_KEPT_PER_HAND;
    pub const CARDS_REQUIRED_IN_CRIB: usize = CARDS_DISCARDED_TO_CRIB * PLAYER_COUNT;

    pub const PLAY_TARGET: usize = 31;

    pub const WINNING_SCORE: usize = 121;
}
