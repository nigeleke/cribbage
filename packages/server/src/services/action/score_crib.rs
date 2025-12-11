use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Acknowledges the current dealer score and goes on to score the crib for the specified user
/// if both players have acknowledged.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user scoring the crib.
/// - `game_id`: The ID of the game containing the crib.
///
/// # Returns
///
/// Returns `Ok(())` if the crib was successfully scored.
/// Returns a `ServerError` if the action is forbidden, the game is not found,
/// the crib cannot be scored yet, or another internal error occurs.
///
/// # Example
///
/// ```no_run
/// score_crib(server_state.clone(), user_id, game_id).await?;
/// println!("Crib scored successfully.");
/// ```
pub async fn score_crib(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("score crib".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::ScoreCrib { player };

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
