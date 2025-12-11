use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Acknowledges the crib score and goes on start the next deal / discarding round
/// if both players have acknowledged.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user starting the next round.
/// - `game_id`: The ID of the game to advance.
///
/// # Returns
///
/// Returns `Ok(())` if the next round was successfully started.
/// Returns a `ServerError` if the action is forbidden, the game is not found,
/// or another internal error occurs.
pub async fn start_next_round(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("start next round".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::StartNextRound { player };

        server_state
            .cqrs
            .execute(&aggregate_id, command)
            .await
            .map_err(bug!())?;

        Ok(())
    } else {
        Err(ServerError::NotFound)
    }
}
