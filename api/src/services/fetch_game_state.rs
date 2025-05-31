use crate::{
    dto::{ActiveGameId, GameState, UserId},
    set_no_cache_response,
};
use dioxus::prelude::*;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{api_state::ApiState, database::select_active_game};
}

#[cfg(feature = "server")]
use server::*;

#[server]
pub async fn fetch_game_state(
    id: ActiveGameId,
    user_id: UserId,
) -> Result<GameState, ServerFnError> {
    set_no_cache_response!();
    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let game = select_active_game(pool, id.value()).await?;
    let game_dto = GameState::try_from(game, user_id)?;
    Ok(game_dto)
}
