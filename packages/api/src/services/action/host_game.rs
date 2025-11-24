use crate::dto::{GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[post("/api/{user_id}/host_game/", State(server_state): State<server::ServerState>)]
pub async fn host_game(user_id: UserIdDTO) -> Result<GameIdDTO> {
    use server::action::host_game;
    use server::domain::UserId;

    let user_id = UserId::from(user_id.value());

    let game_id = host_game(server_state, user_id).await?;
    Ok(GameIdDTO::from(game_id.value()))
}
