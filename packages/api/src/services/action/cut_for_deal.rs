use dioxus::{fullstack::extract::State, prelude::*};

use crate::{
    dto::{GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/game/{game_id}/cut_for_deal", State(server_state): State<server::ServerState>)]
pub async fn cut_for_deal(user_id: UserIdDTO, game_id: GameIdDTO) -> Result<(), ApiError> {
    use server::{
        action::cut_for_deal,
        domain::{GameId, UserId},
    };

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let _ = cut_for_deal(server_state, user_id, game_id).await?;
    Ok(())
}
