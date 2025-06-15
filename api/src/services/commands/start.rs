use crate::{UserId, dto::GameId};
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        ApiState, ServiceError,
        database::{select_active_game, update_active_game_state},
        services::redis::game_start_state_key,
    };
    pub use deadpool_redis::redis::{AsyncCommands, Commands};
    pub use domain::{HasState, State};
    pub use std::{collections::HashMap, sync::Arc};
}

#[cfg(feature = "server")]
use server::*;

#[server]
pub async fn start(game_id: GameId, user_id: UserId) -> Result<bool, ServerFnError> {
    use crate::services::redis::{HDeleteEvents, HGetEvents, HSetEvent};

    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();
    let redis = context.redis();

    let key = game_start_state_key(game_id);

    redis.hset_event(&key, &user_id.to_string(), &true).await?;

    let start_statuses: HashMap<String, bool> = redis.hget_events(&key).await?;

    let all_ready = start_statuses.len() == domain::NUMBER_OF_PLAYERS_IN_GAME
        && start_statuses.values().all(|&status| status);

    if all_ready {
        let mut tx = pool.begin().await?;
        let game = select_active_game(tx.as_mut(), game_id.value()).await?;
        let state = serde_json::from_value::<State>(game.state.0)?;

        let game = match state {
            State::Starting(game) => Ok(game.start()?),
            other => Err(ServiceError::InvalidState(other.as_ref().to_string())),
        }?;

        let state = serde_json::to_value(game.state())?;

        let _ = update_active_game_state(tx.as_mut(), game_id.value(), &state).await?;

        redis.hdelete_events(&key).await?;

        tx.commit().await?;
    }

    Ok(all_ready)
}
