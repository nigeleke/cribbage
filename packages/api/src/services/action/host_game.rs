#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::ServerStateExtractor;
use crate::{
    dto::{GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/host_game/", State(server_state): State<ServerStateExtractor>)]
pub async fn host_game(user_id: UserIdDTO) -> Result<GameIdDTO, ApiError> {
    use server::{action::host_game, domain::UserId};

    let user_id = UserId::from(user_id.value());

    let game_id = host_game(server_state.0, user_id).await?;
    Ok(GameIdDTO::from(game_id.value()))
}
