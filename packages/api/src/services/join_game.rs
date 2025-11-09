use crate::{GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[post("/api/{user_id}/game/{game_id}/join", State(server_state): State<server::ServerState>)]
pub async fn join_game(user_id: UserIdDTO, game_id: GameIdDTO) -> Result<(), ServerFnError> {
    let user_id = server::UserId::from(user_id.value());
    let game_id = server::GameId::from(game_id.value());

    server::join_game(server_state, user_id, game_id)
        .await
        .map_err(ServerFnError::new)
}
