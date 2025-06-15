use crate::dto::{GameId, UserId};
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        ApiState,
        database::{ActiveGameRow, delete_lobby_game, insert_active_game, select_lobby_game},
    };
    pub use domain::HasState;
    pub use std::sync::Arc;
}

#[cfg(feature = "server")]
use server::*;

#[server]
pub async fn activate_game(
    user_id: UserId,
    lobby_game_id: GameId,
) -> Result<GameId, ServerFnError> {
    use domain::{Game, Player, Players};

    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let lobby_game_id = *lobby_game_id.value();
    let lobby_game = select_lobby_game(pool, lobby_game_id).await?;
    let user1 = lobby_game.owner_id;
    let user2 = *user_id.value();

    let players = Players::from_iter([user1, user2].map(Player::from));
    let game = Game::<_>::try_new(&players).map_err(ServerFnError::WrappedServerError)?;
    let state = serde_json::to_value(game.state())?;

    let active_game = ActiveGameRow::new(lobby_game_id, lobby_game.name, user1, user2, state);

    let mut tx = pool.begin().await?;
    let active_game_id = insert_active_game(tx.as_mut(), &active_game).await?;
    let _ = delete_lobby_game(tx.as_mut(), lobby_game_id).await?;
    let _ = tx.commit().await?;

    Ok(GameId::from(active_game_id))
}
