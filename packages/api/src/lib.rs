#![feature(coverage_attribute)]

//! This crate contains all shared fullstack server functions.
mod dto;
mod services;

pub use {dto::*, services::*};

#[cfg(feature = "server")]
mod convertors;

#[cfg(feature = "server")]
pub use server::initialize_server_state;
