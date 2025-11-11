// mod cut_for_deal;
// mod game_stream;
// mod games_stream;
mod get_available_games;
// mod get_game;
mod available_game_events;
mod events;
mod game_events;
mod host_game;
mod join_game;
mod play_computer;

// pub use cut_for_deal::cut_for_deal;
// pub use game_stream::game_stream;
// pub use games_stream::{Event as GamesStreamEvent, games_stream};
pub use get_available_games::get_available_games;
// pub use get_game::get_game;
pub use available_game_events::{AvailableGameEvent, available_game_events};
pub use game_events::game_events;
pub use host_game::host_game;
pub use join_game::join_game;
pub use play_computer::play_computer;

type AggregateId = String;
