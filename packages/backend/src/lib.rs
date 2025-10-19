#![feature(coverage_attribute)]

mod database;
mod display;
mod domain;
mod error;
mod macros;
mod name_builder;
mod server_state;
mod services;
#[cfg(test)]
mod test;

pub use domain::{GameId, UserId};
pub use server_state::{SERVER_STATE, ServerState};
pub use services::*;
