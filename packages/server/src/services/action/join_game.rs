use dioxus::prelude::*;

use crate::domain::{GameCommand, GameId, UserId};
use crate::server_state::ServerState;
use crate::services::error::ServiceError;

pub async fn join_game(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServiceError> {
    let aggregate_id = game_id.value().to_string();

    let command = GameCommand::JoinGame { user_id };
    let _ = server_state.cqrs.execute(&aggregate_id, command).await?;

    Ok(())
}
