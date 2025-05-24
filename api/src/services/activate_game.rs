use std::sync::Arc;

use dioxus::prelude::*;
#[cfg(feature = "server")]
use sqlx::PgTransaction;
#[cfg(feature = "server")]
use uuid::Uuid;

use crate::dto::{ActiveGameId, UnstartedGameId, UserId};
#[cfg(feature = "server")]
use crate::{
    ApiState,
    database::{ActiveGameRow, delete_unstarted_game, insert_active_game, select_unstarted_game},
};

#[server]
pub async fn activate_game(
    user_id: UserId,
    unstarted_game_id: UnstartedGameId,
) -> Result<ActiveGameId, ServerFnError> {
    use domain::{Game, Player, Players};

    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let unstarted_game_id = *unstarted_game_id.value();
    let unstarted_game = select_unstarted_game(pool, unstarted_game_id).await?;
    let user1 = unstarted_game.owner_id;
    let user2 = *user_id.value();

    let players = Players::from_iter([user1, user2].map(Player::from));
    let state = Game::<_>::try_new(&players).map_err(ServerFnError::WrappedServerError)?;

    let active_game = ActiveGameRow::new(unstarted_game.name, user1, user2, state);

    let mut tx = pool.begin().await?;
    let active_game_id = insert_active_game(tx.as_mut(), &active_game).await?;
    let _ = delete_unstarted_game(tx.as_mut(), unstarted_game_id).await?;
    let _ = tx.commit();

    Ok(ActiveGameId::from(active_game_id))
}
