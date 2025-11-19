use crate::domain::GameCommand;
use crate::domain::{GameId, UserId};
use crate::error::ServerError;
use crate::server_state::ServerState;

pub async fn play_computer(
    server_state: ServerState,
    user_id: UserId,
) -> Result<GameId, ServerError> {
    let game_id = GameId::new();
    let aggregate_id = game_id.value().to_string();

    let command = GameCommand::PlayComputer { user_id, game_id };
    let _ = server_state
        .cqrs
        .execute(&aggregate_id, command)
        .await
        .map_err(ServerError::bug)?;

    Ok(game_id)
}
