#![feature(coverage_attribute)]

mod error;
mod get_available_games;
mod host_game;
mod macros;
mod user_game_stream;

pub use error::ApiError;
pub use get_available_games::{Response as AvailableGamesResponse, Since, get_available_games};
pub use host_game::host_game;
pub use user_game_stream::user_game_stream;
