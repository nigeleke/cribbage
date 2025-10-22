use dioxus::prelude::*;
use dto::{GameIdDTO, UserIdDTO};

#[post("/api/{user_id}/game/{game_id}/join")]
pub async fn join_game(user_id: UserIdDTO, game_id: GameIdDTO) -> Result<GameIdDTO, ServerFnError> {
    use backend::{GameId, UserId};

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let game_id = backend::join_game(user_id, game_id)
        .await
        .map_err(ServerFnError::new)?;

    Ok(GameIdDTO::from(game_id.value()))
}
