#![feature(coverage_attribute)]

//! This crate contains all shared fullstack server functions.
mod dto;
mod services;
// #[cfg(test)]
// mod test;

pub use dto::*;
pub use services::*;

#[cfg(feature = "server")]
pub use server::initialize_server_state;
