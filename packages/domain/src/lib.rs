#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

/// Fixed quantities and scoring rules used throughout a game of cribbage.
pub mod constants;

mod card;
mod cards;
mod game;
mod players;
mod plays;
mod scoreboard;
mod types;

#[cfg(test)]
mod macros;

/// Commonly used domain types.
///
/// This module provides a convenient set of imports for working with a game.
pub mod prelude {
    pub use super::card::{Card, Face, Suit};
    pub use super::cards::{Crib, Deck, Hand};
    pub use super::game::{
        CutForDealOutcome, CutStarterOutcome, Cutting, DealOutcome, Dealing, DiscardOutcome,
        Discarding, Finished, Game, GameError, GoOutcome, PlayOutcome, Playing, ScoreCribOutcome,
        ScoreDealerOutcome, ScorePoneOutcome, Scoring, ScoringCrib, ScoringDealer, ScoringPone,
        Starting,
    };
    pub use super::players::Player;
    pub use super::plays::{Play, PlayState};
    pub use super::scoreboard::{Call, Pegs, Points, Score, Scoreboard};
    pub use super::types::{Discard, Hands};
}
