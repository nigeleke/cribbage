//! Services module providing the core HTTP endpoint logic for the game API.
//!
//! This module is organized into three submodules:
//!
//! - `action`: Handles state-modifying requests (playing cards, joining games, scoring, etc.).
//! - `queries`: Handles read-only requests (fetching available games, game state, etc.).
//! - `stream`: Handles streaming endpoints for real-time updates.

/// Actions that modify the game state.
pub mod action;

/// Queries that retrieve game or user data.
pub mod queries;

/// Streaming endpoints for real-time updates.
pub mod stream;
