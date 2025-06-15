use crate::{
    dto::{GameId, UserGameState, UserId},
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
    game_id: GameId,
    user_id: UserId,
) -> Result<UserGameState, ServerFnError> {
    set_no_cache_response!();
    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let game = select_active_game(pool, game_id.value()).await?;
    let game_dto = UserGameState::try_from(game, user_id)?;
    Ok(game_dto)
}
