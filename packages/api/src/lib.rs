#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![doc = include_str!("../README.md")]

/// Data Transfer Object (DTO) definitions used by the `services` layer.
pub mod dto;

/// Error types used throughout the `services` module.
pub mod error;

mod services;

pub use services::*;

#[cfg(feature = "server")]
mod extracted_server_state;

#[cfg(feature = "server")]
pub use {extracted_server_state::ServerStateExtractor, server::initialize_server_state};
