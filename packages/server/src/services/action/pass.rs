use crate::{
    domain::{GameCommand, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
    services::view::get_game,
};

pub async fn pass(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    if let Some(game) = get_game(server_state.clone(), game_id).await? {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("pass".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::Pass { player };
        let _ = server_state
            .cqrs
            .execute(&aggregate_id, command)
            .await
            .map_err(ServerError::bug)?;

        Ok(())
    } else {
        Err(ServerError::NotFound)
    }
}
