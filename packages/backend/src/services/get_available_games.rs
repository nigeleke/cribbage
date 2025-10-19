use std::str::FromStr;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use strum::{AsRefStr, EnumString};

use crate::database::select_available_games;
use crate::domain::{GameId, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;

#[derive(Debug, AsRefStr, EnumString)]
pub enum AvailableGameSource {
    Lobby,
    Active,
}

pub async fn get_available_games(
    user_id: UserId,
    filter: String,
    last_created_at: Option<DateTime<Utc>>,
) -> Result<
    (
        Vec<(GameId, AvailableGameSource, String)>,
        bool,
        Option<DateTime<Utc>>,
    ),
    BackendError,
> {
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
            let game_id = GameId::from(row.id);
            let source = AvailableGameSource::from_str(&row.source)?;
            let name = row.name;
            Ok::<_, BackendError>((game_id, source, name))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((games, chunk.has_more, chunk.last_created_at))
}
