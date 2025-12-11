use crate::{
    domain::{GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
};

/// Joins an existing game as a guest.
///
/// This function allows a user to join a game that has been hosted by another user.
/// The game must have an available slot for a guest. After joining, the game state
/// is updated accordingly.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user who wants to join the game.
/// - `game_id`: The ID of the game to join.
///
/// # Returns
///
/// Returns `Ok(())` if the user successfully joined the game.
/// Returns a `ServerError` if the game is not found, the user is forbidden from joining,
/// or another internal error occurs.
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
