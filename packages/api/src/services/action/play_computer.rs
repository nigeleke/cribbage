use crate::{GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[post("/api/{user_id}/play_computer/", State(server_state): State<server::ServerState>)]
pub async fn play_computer(user_id: UserIdDTO) -> Result<GameIdDTO, ServerFnError> {
    let user_id = server::UserId::from(user_id.value());

    server::action::play_computer(server_state, user_id)
        .await
        .map(|id| GameIdDTO::from(id.value()))
        .map_err(ServerFnError::new)
}
