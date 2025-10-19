use dioxus::prelude::*;
use serde_json::json;

use crate::SERVER_STATE;
use crate::database::{NewGame, insert_game};
use crate::domain::{Game, GameId, UserId};
use crate::error::BackendError;
use crate::name_builder::generate_game_name;

pub async fn host_game(user_id: UserId) -> Result<GameId, BackendError> {
    let game = Game::host_game(user_id, generate_game_name());

    let name = game.name().clone();
    let host_id = game.host().value();
    let guest_id = game.guest().map(|id| id.value());
    let state = json!(game.state()).into();

    let new_game = NewGame {
        name,
        host_id,
        guest_id,
        state,
    };

    let game_id = insert_game(SERVER_STATE.postgres_pool(), &new_game).await?;
    let game_id = GameId::from(game_id);

    Ok(game_id)
}
