use dioxus::prelude::*;

use crate::convertors;
use crate::database::select_game;
use crate::domain::{Game, GameId};

use crate::server_state::ServerState;
use crate::services::error::ServiceError;

pub async fn get_game(
    server_state: ServerState,
    game_id: GameId,
) -> Result<Option<Game>, ServiceError> {
    let pool = server_state.pool.clone();

    let game = select_game(&*pool, game_id.value()).await?;
    let game = game.map(convertors::game_row_to_game).transpose()?;

    Ok(game)
}
