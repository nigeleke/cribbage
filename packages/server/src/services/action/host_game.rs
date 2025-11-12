use dioxus::prelude::*;

use crate::GameCommand;
use crate::domain::{GameId, UserId};
use crate::server_state::ServerState;
use crate::services::error::ServiceError;

pub async fn host_game(server_state: ServerState, user_id: UserId) -> Result<GameId, ServiceError> {
    let game_id = GameId::new();
    let aggregate_id = game_id.value().to_string();

    let command = GameCommand::HostGame { user_id, game_id };
    let _ = server_state.cqrs.execute(&aggregate_id, command).await?;

    Ok(game_id)
}
