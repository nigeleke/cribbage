use crate::{UserId, dto::UnstartedGame};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{api_state::ApiState, database::select_unstarted_games, set_default_cache};
    pub use futures::StreamExt;
    pub use redis::AsyncCommands;
    pub use tokio::sync::OnceCell;
    pub const REDIS_CHANNEL: &str = "unstarted_games_change";
    pub static DATABASE_LISTENER: OnceCell<()> = OnceCell::const_new();
}

#[cfg(feature = "server")]
use server::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    last_created_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Request {
    user: UserId,
    filter: String,
    #[serde(default)]
    state: State,
}

impl Request {
    pub fn new(user: UserId, filter: String, state: State) -> Self {
        Self {
            user,
            filter,
            state,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response {
    games: Vec<UnstartedGame>,
    has_more: bool,
    #[serde(default)]
    state: State,
}

impl Response {
    pub fn games(&self) -> &Vec<UnstartedGame> {
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
pub async fn fetch_unstarted_games(request: Request) -> Result<Response, ServerFnError> {
    const CHUNK_SIZE: u32 = 20;

    set_default_cache!();
    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let filter = (!request.filter.is_empty()).then_some(request.filter);
    let chunk = select_unstarted_games(
        pool,
        CHUNK_SIZE,
        request.state.last_created_at,
        filter,
        *request.user.value(),
    )
    .await?;

    let game_dtos = chunk
        .games
        .into_iter()
        .map(UnstartedGame::from)
        .collect::<Vec<_>>();

    let response = Response {
        games: game_dtos,
        has_more: chunk.has_more,
        state: State {
            last_created_at: chunk.last_created_at,
        },
    };

    Ok(response)
}
