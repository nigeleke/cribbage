use crate::dto::{UnstartedGame, UserId};
use dioxus::prelude::*;
use std::sync::Arc;

#[server]
pub async fn new_human_game(user_id: UserId) -> Result<UnstartedGame, ServerFnError> {
    use crate::{
        ApiState,
        database::{UnstartedGameRow, insert_unstarted_game},
    };

    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let game = UnstartedGameRow::new(*user_id.value());
    let game = insert_unstarted_game(pool, &game).await?;
    let game = UnstartedGame::from(game);

    Ok(game)
}
