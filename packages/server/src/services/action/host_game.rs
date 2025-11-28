use crate::{
    bug,
    domain::{GameCommand, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
};

pub async fn host_game(server_state: ServerState, user_id: UserId) -> Result<GameId, ServerError> {
    let game_id = GameId::new();
    let aggregate_id = game_id.value().to_string();

    let command = GameCommand::HostGame { user_id, game_id };

    server_state
        .cqrs
        .execute(&aggregate_id, command)
        .await
        .map_err(bug!())?;

    Ok(game_id)
}
