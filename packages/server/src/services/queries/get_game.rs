use crate::{
    domain::{Game, GameId},
    error::{ServerError, bug},
    server_state::ServerState,
};

/// Retrieves a game by its ID.
///
/// This function fetches the specified game from the server state. If the
/// game exists, it is returned; otherwise, `None` is returned.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the games database.
/// - `game_id`: The ID of the game to retrieve.
///
/// # Returns
///
/// Returns `Ok(Some(Game))` if the game exists.
/// Returns `Ok(None)` if no game with the given ID is found.
/// Returns a `ServerError` if there is a problem accessing the database
/// or processing the request.
pub async fn get_game(
    server_state: ServerState,
    game_id: GameId,
) -> Result<Option<Game>, ServerError> {
    use cqrs_es::persist::ViewRepository;

    let game = server_state
        .game_view_repo
        .load(&game_id.value().to_string())
        .await
        .map_err(bug!())?;

    let game = game.map(|g| g.instance().clone());

    Ok(game)
}
