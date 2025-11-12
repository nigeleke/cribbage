use crate::dto::{GameIdDTO, UserGameDTO, UserIdDTO};

use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[get("/api/{user_id}/game/{game_id}", State(server_state): State<server::ServerState>)]
pub async fn get_game(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<UserGameDTO, ServerFnError> {
    use crate::convertors;
    use server::GameId;
    use server::UserId;

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let game = server::view::get_game(server_state, game_id)
        .await
        .map_err(ServerFnError::new)?;

    let game = game.map_or(UserGameDTO::default(), |game| {
        convertors::game_to_user_game_dto(&game, &user_id)
    });

    Ok(game)
}
