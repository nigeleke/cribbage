#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

use crate::{
    dto::{GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/game/{game_id}/acknowledge_plays_ended", State(server_state): State<server::ServerState>)]
pub async fn acknowledge_plays_ended(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<(), ApiError> {
    use server::{
        action::acknowledge_plays_ended,
        domain::{GameId, UserId},
    };

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    acknowledge_plays_ended(server_state, user_id, game_id).await?;
    Ok(())
}
