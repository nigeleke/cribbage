mod activate_game;
mod active_games_stream;
mod error;
mod fetch_available_games;
mod fetch_game_state;
mod fetch_unstarted_game;
mod macros;
mod new_computer_game;
mod new_human_game;
#[cfg(feature = "server")]
mod redis;
mod started_game_stream;
mod unstarted_games_stream;

pub use activate_game::activate_game;
pub use active_games_stream::{Event as ActiveGamesEvent, active_games_stream};
pub use fetch_available_games::{
    Request as AvailableGamesRequest, Response as AvailableGamesResponse,
    State as AvailableGamesState, fetch_available_games,
};
pub use fetch_game_state::fetch_game_state;
pub use fetch_unstarted_game::fetch_unstarted_game;
pub use new_computer_game::new_computer_game;
pub use new_human_game::new_human_game;
pub use started_game_stream::{Event as StartedGameEvent, started_game_stream};
pub use unstarted_games_stream::{Event as UnstartedGamesEvent, unstarted_games_stream};
