use crate::{
    dto::{UnstartedGame, UnstartedGameId},
    set_default_cache,
};
use dioxus::prelude::*;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{api_state::ApiState, database::select_unstarted_game};
}

#[cfg(feature = "server")]
use server::*;

#[server]
pub async fn fetch_unstarted_game(id: UnstartedGameId) -> Result<UnstartedGame, ServerFnError> {
    set_default_cache!();
    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let game = select_unstarted_game(pool, *id.value()).await?;
    let game_dto = UnstartedGame::from(game);
    Ok(game_dto)
}
