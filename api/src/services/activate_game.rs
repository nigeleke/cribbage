use crate::dto::{ActiveGameId, UnstartedGameId, UserId};
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        ApiState,
        database::{
            ActiveGameRow, StartedGameRow, delete_unstarted_game, insert_active_game,
            insert_started_game, select_unstarted_game,
        },
    };
    pub use domain::HasState;
    pub use std::sync::Arc;
}

#[cfg(feature = "server")]
use server::*;

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
    let game = Game::<_>::try_new(&players).map_err(ServerFnError::WrappedServerError)?;
    let state = serde_json::to_value(game.state())?;

    let active_game = ActiveGameRow::new(unstarted_game.name, user1, user2, state);
    let started_game = StartedGameRow::new(unstarted_game_id, active_game.id);

    let mut tx = pool.begin().await?;
    let active_game_id = insert_active_game(tx.as_mut(), &active_game).await?;
    let _ = insert_started_game(tx.as_mut(), &started_game).await?;
    let _ = delete_unstarted_game(tx.as_mut(), unstarted_game_id).await?;
    let _ = tx.commit().await?;

    Ok(ActiveGameId::from(active_game_id))
}
