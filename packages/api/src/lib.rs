#![feature(coverage_attribute)]

//! This crate contains all shared fullstack server functions.
mod dto;
mod error;
mod services;

pub use {dto::*, error::*, services::*};

#[cfg(feature = "server")]
pub use server::initialize_server_state;
