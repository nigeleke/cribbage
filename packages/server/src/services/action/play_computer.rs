use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
};

/// Starts a game against the computer for the specified user.
///
/// This function creates a new game where the user plays against an automated
/// opponent. The game is initialized in the starting phase, and the
/// returned `GameId` can be used to track or manage the game.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user who will play against the computer.
///
/// # Returns
///
/// Returns `Ok(GameId)` containing the ID of the newly created game.
/// Returns a `ServerError` if the game cannot be created due to internal issues.
pub async fn play_computer(
    server_state: ServerState,
    user_id: UserId,
) -> Result<GameId, ServerError> {
    let game_id = GameId::new();
    let aggregate_id = game_id.value().to_string();

    let command = GameCommand::PlayComputer { user_id, game_id };

    server_state
        .cqrs
        .execute(&aggregate_id, command)
        .await
        .map_err(bug!())?;

    Ok(game_id)
}
