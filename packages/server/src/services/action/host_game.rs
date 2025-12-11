use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
};

/// Hosts a new game for the specified user.
///
/// This function creates a new game with the user as the host. The game
/// is initialized in the starting phase, and the game ID is returned
/// so it can be used to join or manage the game.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user who will host the new game.
///
/// # Returns
///
/// Returns `Ok(GameId)` containing the ID of the newly created game.
/// Returns a `ServerError` if the game cannot be created due to internal issues.
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
