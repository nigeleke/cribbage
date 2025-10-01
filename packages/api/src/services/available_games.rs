use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dto::{AvailableGameDTO, UserIdDTO};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    last_created_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Request {
    user: UserIdDTO,
    filter: String,
    #[serde(default)]
    state: State,
}

impl Request {
    pub fn new(user: UserIdDTO, filter: String, state: State) -> Self {
        Self {
            user,
            filter,
            state,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response {
    games: Vec<AvailableGameDTO>,
    has_more: bool,
    #[serde(default)]
    state: State,
}

impl Response {
    pub fn games(&self) -> &[AvailableGameDTO] {
        &self.games
    }

    pub fn has_more(&self) -> bool {
        self.has_more
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

#[server]
pub async fn get_available_games(request: Request) -> Result<Response, ServerFnError> {
    use axum::http;

    use crate::{ApiState, select_available_games, set_no_cache_response};

    const CHUNK_SIZE: u32 = 20;

    set_no_cache_response!();

    let context = dioxus::server::context::server_context()
        .get::<ApiState>()
        .expect("server context should be initialised with ApiState");

    let postgres_pool = context.postgres_pool();

    let filter = (!request.filter.is_empty()).then_some(request.filter);
    let chunk = select_available_games(
        postgres_pool,
        CHUNK_SIZE,
        request.state.last_created_at,
        filter,
        request.user.value(),
    )
    .await?;

    let game_dtos = chunk
        .games
        .into_iter()
        .map(AvailableGameDTO::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let response = Response {
        games: game_dtos,
        has_more: chunk.has_more,
        state: State {
            last_created_at: chunk.last_created_at,
        },
    };

    Ok(response)
}
