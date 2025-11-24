use crate::dto::{GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[post("/api/{user_id}/game/{game_id}/join_game", State(server_state): State<server::ServerState>)]
pub async fn join_game(user_id: UserIdDTO, game_id: GameIdDTO) -> Result<()> {
    use server::action::join_game;
    use server::domain::{GameId, UserId};

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let _ = join_game(server_state, user_id, game_id).await?;
    Ok(())
}
