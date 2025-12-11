use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Acknowledges the cuts for deal, then starts a hosted game for the specified user.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user starting the game.
/// - `game_id`: The ID of the game to start.
///
/// # Returns
///
/// Returns `Ok(())` if the game was successfully started.
/// Returns a `ServerError` if the action is forbidden, the game is not found,
/// or another internal error occurs.
///
/// # Example
///
/// ```no_run
/// start_game(server_state.clone(), user_id, game_id).await?;
/// println!("Game started successfully.");
/// ```
pub async fn start_game(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("start game".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::StartGame { player };

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
