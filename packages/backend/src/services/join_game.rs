use dioxus::prelude::*;

use crate::database::{UpdateGame, select_game, update_game};
use crate::domain::{GameId, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::{convertors, get_game};

pub async fn join_game(user_id: UserId, game_id: GameId) -> Result<GameId, BackendError> {
    let game = select_game(SERVER_STATE.postgres_pool(), game_id.value())
        .await?
        .ok_or_else(|| BackendError::GameNotFound(game_id))?;

    let game = convertors::game_row_to_game(game)?;
    let game = game.join_game(user_id)?;

    let update = UpdateGame {
        id: game_id.value(),
        guest_id: Some(user_id.value()),
        ..Default::default()
    };

    let _ = update_game(SERVER_STATE.postgres_pool(), &update).await?;

    Ok(*game.id())
}
