#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod card;
mod cards;
mod constants;
mod game;
mod players;
mod plays;
mod scoreboard;
mod types;

pub use card::{Card, Face, Rank, Suit, Value};
pub use cards::{Crib, Deck, Hand};
pub use game::Game;
pub use players::{Dealer, PLAYERS, Player, Players, Pone, Roles};
pub use plays::{GoStatus, Play, PlayState};
pub use scoreboard::{Call, Event as ScoreEvent, Points, Scoreboard};
pub use types::{CutsForDeal, Discard, Discards, Hands};
