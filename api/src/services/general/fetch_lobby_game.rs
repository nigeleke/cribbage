use crate::{
    dto::{GameId, LobbyGame},
    set_no_cache_response,
};
use dioxus::prelude::*;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{api_state::ApiState, database::select_lobby_game};
}

#[cfg(feature = "server")]
use server::*;

#[server]
pub async fn fetch_lobby_game(id: GameId) -> Result<LobbyGame, ServerFnError> {
    set_no_cache_response!();
    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let game = select_lobby_game(pool, *id.value()).await?;
    let game_dto = LobbyGame::from(game);
    Ok(game_dto)
}
