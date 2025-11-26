use chrono::{DateTime, Utc};

use crate::{
    bug,
    convertors::available_game_row_to_available_game,
    database::select_available_games,
    domain::{AvailableGame, UserId},
    error::ServerError,
    server_state::ServerState,
};

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
