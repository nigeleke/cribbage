#![feature(coverage_attribute)]

mod convertors;
mod database;
mod display;
pub mod domain;
mod error;
mod macros;
mod name_builder;
mod projections;
mod server_state;
mod services;

pub use error::ServerError;
pub use server_state::{ServerState, initialize_server_state};
pub use services::*;
