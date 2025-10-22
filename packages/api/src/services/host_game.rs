use dioxus::prelude::*;
use dto::{GameIdDTO, UserIdDTO};

#[post("/api/host_game/")]
pub async fn host_game(user_id: UserIdDTO) -> Result<GameIdDTO, ServerFnError> {
    let user_id = backend::UserId::from(user_id.value());

    let game_id = backend::host_game(user_id)
        .await
        .map_err(ServerFnError::new)?;

    Ok(GameIdDTO::from(game_id.value()))
}
