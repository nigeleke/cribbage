use crate::{GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[post("/api/{user_id}/play_computer/", State(server_state): State<server::ServerState>)]
pub async fn play_computer(user_id: UserIdDTO) -> Result<GameIdDTO> {
    use server::action::play_computer;
    use server::domain::UserId;

    let user_id = UserId::from(user_id.value());

    let game_id = play_computer(server_state, user_id).await?;
    Ok(GameIdDTO::from(game_id.value()))
}
