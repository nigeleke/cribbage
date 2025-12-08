#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::ServerStateExtractor;
use crate::{
    dto::{GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/game/{game_id}/score_pone", State(server_state): State<ServerStateExtractor>)]
pub async fn score_pone(user_id: UserIdDTO, game_id: GameIdDTO) -> Result<(), ApiError> {
    use server::{
        action::score_pone,
        domain::{GameId, UserId},
    };

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    score_pone(server_state.0, user_id, game_id).await?;
    Ok(())
}
