#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(rust_2018_idioms)]
#![doc = include_str!("../README.md")]

mod commands;
mod display;
mod domain;
mod error;
mod events;
mod macros;
mod name_builder;
mod reactors;
mod state;
mod types;

#[cfg(test)]
mod test;

#[cfg(test)]
pub(crate) use self::test::{GameBuilder, GameTestFramework};
pub(crate) use self::{
    commands::*, domain::*, error::*, events::*, reactors::*, state::*, types::*,
};

pub(crate) mod constants {
    /** Cribbage can be a two, three or (at a push) four player game. This implementation is for two
     * players.
     */
    pub const PLAYER_COUNT: usize = 2;

    /** Six [Card]s are dealt to each Player by the current dealer. */
    pub const CARDS_DEALT_PER_HAND: usize = 6;

    /** [Player]s discard two cards each into the [Crib] leaving four for [Score]ing and [Plays]. */
    pub const CARDS_KEPT_PER_HAND: usize = 4;
    pub const CARDS_DISCARDED_TO_CRIB: usize = CARDS_DEALT_PER_HAND - CARDS_KEPT_PER_HAND;

    /** Each [Player] discarding two [Card]s to the [Crib] will mean four [Card]s end up there. */
    pub const CARDS_REQUIRED_IN_CRIB: usize = CARDS_DISCARDED_TO_CRIB * PLAYER_COUNT;

    /** Each [Plays.Play] cannot have a running total of more than 31. */
    pub const PLAY_TARGET: usize = 31;

    /** Short games can play to 61, but it is normal to play to 121, as in this implementation. */
    pub const WINNING_SCORE: usize = 121;
}
