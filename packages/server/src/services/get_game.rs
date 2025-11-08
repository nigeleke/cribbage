use dioxus::prelude::*;

use crate::database::select_game;
use crate::domain::{Game, GameId, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

pub async fn get_game(_user_id: UserId, game_id: GameId) -> Result<Option<Game>, BackendError> {
    let game = select_game(SERVER_STATE.postgres_pool(), game_id.value()).await?;

    let game = game.map(convertors::game_row_to_game).transpose()?;

    Ok(game)
}
