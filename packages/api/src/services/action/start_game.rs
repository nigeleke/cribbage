#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::ServerStateExtractor;
use crate::{
    dto::{GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/game/{game_id}/start_game", State(server_state): State<ServerStateExtractor>)]
pub async fn start_game(user_id: UserIdDTO, game_id: GameIdDTO) -> Result<(), ApiError> {
    use server::{
        action::start_game,
        domain::{GameId, UserId},
    };

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    start_game(server_state.0, user_id, game_id).await?;
    Ok(())
}
