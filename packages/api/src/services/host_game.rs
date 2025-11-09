use crate::{GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[post("/api/{user_id}/host_game/", State(server_state): State<server::ServerState>)]
pub async fn host_game(user_id: UserIdDTO) -> Result<GameIdDTO, ServerFnError> {
    let user_id = server::UserId::from(user_id.value());

    server::host_game(server_state, user_id)
        .await
        .map(|id| GameIdDTO::from(id.value()))
        .map_err(ServerFnError::new)
}
