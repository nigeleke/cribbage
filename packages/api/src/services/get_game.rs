use dioxus::prelude::*;
use dto::{GameIdDTO, UserGameDTO, UserIdDTO};

#[get("/api/{user_id}/game/{game_id}")]
pub async fn get_game(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<Option<UserGameDTO>, ServerFnError> {
    use backend::{GameId, UserId};

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let game = backend::get_game(user_id, game_id)
        .await
        .map_err(ServerFnError::new)?;

    let game = game.as_ref().map(|g| users_view(user_id, g)).transpose()?;

    Ok(game)
}

#[cfg(feature = "server")]
fn users_view(
    user_id: backend::UserId,
    game: &backend::Game,
) -> Result<UserGameDTO, ServerFnError> {
    let name = game.name();
    let game = UserGameDTO::new(name);
    Ok(game)
}
