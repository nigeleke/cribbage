use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Performs a "go" action for the user in a pegging phase of the game.
///
/// In Cribbage, a "go" indicates that the current player cannot play
/// any card without exceeding the running total. This function updates
/// the game state accordingly, advancing the turn or ending the pegging
/// sequence as necessary.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user performing the "go".
/// - `game_id`: The ID of the game in which the action occurs.
///
/// # Returns
///
/// Returns `Ok(())` if the action was successful. Returns a `ServerError` if
/// the action is forbidden, the game is not found, or another internal error occurs.
///
/// # Example
///
/// ```no_run
/// go(server_state.clone(), user_id, game_id).await?;
/// ```
pub async fn go(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    if let Some(game) = get_game(server_state.clone(), game_id).await? {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("go".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::Go { player };

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
