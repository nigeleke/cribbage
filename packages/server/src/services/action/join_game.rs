use crate::{
    bug,
    domain::{GameCommand, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
};

pub async fn join_game(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let aggregate_id = game_id.value().to_string();

    let command = GameCommand::JoinGame { user_id };

    server_state
        .cqrs
        .execute(&aggregate_id, command)
        .await
        .map_err(bug!())?;

    Ok(())
}
