use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Acknowledges the end of plays and goes on to score the pone's hand for the
/// specified user if both players have acknowledged.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user performing the scoring.
/// - `game_id`: The ID of the game containing the pone's hand.
///
/// # Returns
///
/// Returns `Ok(())` if the pone's hand was successfully scored.
/// Returns a `ServerError` if the action is forbidden, the game is not found,
/// or another internal error occurs.
pub async fn score_pone(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("score pone".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::ScorePone { player };

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
