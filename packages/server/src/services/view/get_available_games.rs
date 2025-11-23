use crate::convertors::available_game_row_to_available_game;
use crate::database::select_available_games;
use crate::domain::{AvailableGame, UserId};
use crate::error::ServerError;
use crate::server_state::ServerState;
use chrono::{DateTime, Utc};

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
            .map_err(ServerError::bug)?;

    dioxus::prelude::debug!(
        "server:services:get_available_games: more: {} len: {}",
        chunk.has_more,
        chunk.games.len()
    );

    let games = chunk
        .games
        .into_iter()
        .map(|row| {
            let game = available_game_row_to_available_game(row).map_err(ServerError::bug)?;
            Ok::<_, ServerError>(game)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((games, chunk.has_more, chunk.last_created_at))
}
