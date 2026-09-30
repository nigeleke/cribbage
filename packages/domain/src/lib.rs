#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod card;
mod cards;
mod game;
mod players;
mod scoreboard;
mod types;

pub(crate) use card::*;
pub(crate) use cards::*;
pub(crate) use game::*;
pub(crate) use players::*;
pub(crate) use scoreboard::*;
pub(crate) use types::*;

#[cfg(test)]
mod macros;

pub(crate) mod constants;

/// Commonly used domain types.
///
/// This module provides a convenient set of imports for working with a game.
pub mod prelude {
    pub use super::card::Card;
    pub use super::cards::Deck;
    pub use super::game::{
        CutForDealOutcome, CutStarterOutcome, Cutting, DealOutcome, Dealing, DiscardOutcome,
        Discarding, Finished, Game, GameError, GoOutcome, PlayOutcome, Playing, ScoreCrib,
        ScoreCribOutcome, ScoreDealer, ScoreDealerOutcome, ScorePone, ScorePoneOutcome, Scoring,
        ScoringCrib, ScoringDealer, ScoringPone, Starting,
    };
    // pub use super::players::Player;
    // pub use super::types::Discard;
}
