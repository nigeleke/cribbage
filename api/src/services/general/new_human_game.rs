use crate::dto::{LobbyGame, UserId};
use dioxus::prelude::*;
use std::sync::Arc;

#[server]
pub async fn new_human_game(user_id: UserId) -> Result<LobbyGame, ServerFnError> {
    use crate::{
        ApiState,
        database::{LobbyGameRow, insert_lobby_game},
    };

    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let game = LobbyGameRow::new(*user_id.value());
    let game = insert_lobby_game(pool, &game).await?;
    let game = LobbyGame::from(game);

    Ok(game)
}
