use dioxus::prelude::*;
use sqlx::types::JsonValue;

use crate::cqrs::Command;
use crate::database::{UpdateGame, append_events, select_game, update_game};
use crate::domain::{GameId, JoinGame, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

pub async fn join_game(user_id: UserId, game_id: GameId) -> Result<GameId, BackendError> {
    let id = game_id.value();
    let game = select_game(SERVER_STATE.postgres_pool(), id).await?;
    let game = game
        .map(convertors::game_row_to_game)
        .transpose()?
        .ok_or(BackendError::GameNotFound(game_id))?;

    let command = JoinGame::new(user_id);
    let (events, updated_game) = command.execute(game).await?;

    let guest_id = updated_game.guest().map(|id| id.value());
    let state: JsonValue = convertors::state_to_json(updated_game.state());

    let update = UpdateGame {
        id,
        guest_id,
        state: Some(state),
        ..Default::default()
    };

    let events = events
        .iter()
        .map(convertors::event_to_json)
        .collect::<Vec<_>>();

    let mut tx = SERVER_STATE.postgres_pool().begin().await?;
    let _ = update_game(tx.as_mut(), &update).await?;
    let _ = append_events(tx.as_mut(), game_id.value(), events).await?;
    let _ = tx.commit().await?;

    Ok(game_id)
}
