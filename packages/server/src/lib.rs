#![feature(coverage_attribute)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

mod convertors;
mod database;
mod display;
mod name_builder;
mod projections;
mod server_state;
mod services;

#[cfg(test)]
pub(crate) mod macros;

/// Domain logic types and helpers.
pub mod domain;

/// Error types for server operations.
pub mod error;

pub use server_state::{ServerState, initialize_server_state};
pub use services::*;
