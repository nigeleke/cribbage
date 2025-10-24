use chrono::{DateTime, Utc};
use dioxus::prelude::*;

use crate::database::select_available_games;
use crate::domain::{AvailableGame, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

pub async fn get_available_games(
    user_id: UserId,
    filter: String,
    last_created_at: Option<DateTime<Utc>>,
) -> Result<(Vec<AvailableGame>, bool, Option<DateTime<Utc>>), BackendError> {
    const CHUNK_SIZE: u32 = 20;

    let filter = (!filter.is_empty()).then_some(filter);

    let chunk = select_available_games(
        SERVER_STATE.postgres_pool(),
        CHUNK_SIZE,
        last_created_at,
        filter,
        user_id.value(),
    )
    .await?;

    let games = chunk
        .games
        .into_iter()
        .map(|row| {
            let game = convertors::available_game_row_to_available_game(row)?;
            Ok::<_, BackendError>(game)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((games, chunk.has_more, chunk.last_created_at))
}
