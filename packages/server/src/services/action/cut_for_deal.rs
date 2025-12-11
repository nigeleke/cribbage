use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Performs the "cut for deal" action for a user in a specific game.
///
/// This function represents the step where a user cuts the deck to determine
/// the dealer for the game. It updates the server state accordingly.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user performing the cut.
/// - `game_id`: The ID of the game in which the cut is performed.
///
/// # Returns
///
/// Returns `Ok(())` on success. Returns a `ServerError` if the action is forbidden,
/// the game is not found, or another internal error occurs.
///
/// # Example
///
/// ```no_run
/// cut_for_deal(server_state.clone(), user_id, game_id).await?;
/// ```
pub async fn cut_for_deal(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("cut for deal".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::CutForDeal { player };

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
