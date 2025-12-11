//! This module contains the server services used by the API.
//!
//! It provides actions, streaming events, and queries.

/// Module for game actions like playing a card or scoring hands.
pub mod action;

/// Module for streams of game or server events.
pub mod stream;

/// Module for queries - read side.
pub mod queries;
