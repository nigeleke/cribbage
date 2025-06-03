use crate::{UserId, dto::ActiveGameId};
use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        ApiState, ServiceError,
        database::{select_user_game, update_active_game_state, update_user_game_state},
    };
    pub use domain::HasState;
    pub use std::sync::Arc;
}

#[cfg(feature = "server")]
use server::*;

#[server]
pub async fn redraw(game_id: ActiveGameId, user_id: UserId) -> Result<(), ServerFnError> {
    use domain::State;

    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool();

    let mut tx = pool.begin().await?;
    let game = select_user_game(tx.as_mut(), game_id.value(), user_id.value()).await?;
    let state = serde_json::from_value::<State>(game.state.0)?;

    let game = match state {
        State::Starting(game) => {
            println!("Redrawing for user {user_id} on game: {:?}", game);
            Ok(game.redraw()?)
        }
        _ => Err(ServiceError::InvalidState("start".to_string())),
    }?;

    let state = serde_json::to_value(game.state())?;

    let _ = update_active_game_state(tx.as_mut(), game_id.value(), &state).await?;
    let _ = update_user_game_state(tx.as_mut(), game_id.value(), user_id.value(), &state).await?;

    let _ = tx.commit().await?;

    Ok(())
}
