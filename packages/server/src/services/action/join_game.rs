use dioxus::prelude::*;

use crate::domain::{GameCommand, GameId, UserId};
use crate::server_state::ServerState;

pub async fn join_game(server_state: ServerState, user_id: UserId, game_id: GameId) -> Result<()> {
    let aggregate_id = game_id.value().to_string();

    let command = GameCommand::JoinGame { user_id };
    let _ = server_state.cqrs.execute(&aggregate_id, command).await?;

    Ok(())
}
