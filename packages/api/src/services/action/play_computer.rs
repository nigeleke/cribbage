#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::ServerStateExtractor;
use crate::{
    dto::{GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/play_computer/", State(server_state): State<ServerStateExtractor>)]
pub async fn play_computer(user_id: UserIdDTO) -> Result<GameIdDTO, ApiError> {
    use server::{action::play_computer, domain::UserId};

    let user_id = UserId::from(user_id.value());

    let game_id = play_computer(server_state.0, user_id).await?;
    Ok(GameIdDTO::from(game_id.value()))
}
