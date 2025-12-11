use chrono::{DateTime, Utc};

use crate::{
    convertors::available_game_row_to_available_game,
    database::select_available_games,
    domain::{AvailableGame, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
};

/// Retrieves a list of available games that the specified user can play or join.
///
/// This function fetches games from the server that the specified user
/// is eligible to join. Results can be filtered by a search string and
/// paginated using the `last_created_at` parameter.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the games database.
/// - `user_id`: The ID of the user requesting the available games.
/// - `filter`: A string used to filter games by name or other criteria.
/// - `last_created_at`: Optional timestamp for pagination; only games created
///   after this timestamp are returned.
///
/// # Returns
///
/// Returns a tuple `(games, has_more, last_created_at)`:
/// - `games`: A `Vec<AvailableGame>` containing the available games matching
///   the filter and pagination criteria.
/// - `has_more`: `true` if there are more games to fetch beyond this page.
/// - `last_created_at`: The timestamp of the last game returned, to be used
///   for the next paginated request.
///
/// Returns a `ServerError` if there is a problem accessing the database or
/// processing the request.
///
/// # Example
///
/// ```no_run
/// let (games, has_more, last) = get_available_games(
///     server_state.clone(),
///     user_id,
///     "cribbage".to_string(),
///     None
/// ).await?;
/// println!("Found {} available games.", games.len());
/// ```
pub async fn get_available_games(
    server_state: ServerState,
    user_id: UserId,
    filter: String,
    last_created_at: Option<DateTime<Utc>>,
) -> Result<(Vec<AvailableGame>, bool, Option<DateTime<Utc>>), ServerError> {
    let pool = server_state.pool.clone();
    const CHUNK_SIZE: u32 = 5;

    let filter = (!filter.is_empty()).then_some(filter);

    let chunk =
        select_available_games(&*pool, CHUNK_SIZE, last_created_at, filter, user_id.value())
            .await
            .map_err(bug!())?;

    let games = chunk
        .games
        .into_iter()
        .map(|row| {
            let game = available_game_row_to_available_game(row).map_err(bug!())?;
            Ok::<_, ServerError>(game)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((games, chunk.has_more, chunk.last_created_at))
}
