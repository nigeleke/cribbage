#![feature(coverage_attribute)]

//! This crate contains all shared fullstack server functions.
pub mod dto;
pub mod error;
mod services;

pub use services::*;

#[cfg(feature = "server")]
pub use server::initialize_server_state;
