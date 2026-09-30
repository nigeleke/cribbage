#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod game;
mod user;

pub(crate) use game::GameId;
pub(crate) use user::UserId;

/// This module provides a convenient set of imports for working with a tellus actor.
pub mod prelude {
    // pub use super::GameId;
    // pub use super::UserId;
}
