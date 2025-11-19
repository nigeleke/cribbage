use crate::dto::{GameIdDTO, UserGameDTO, UserIdDTO};

use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[get("/api/{user_id}/game/{game_id}", State(server_state): State<server::ServerState>)]
pub async fn get_game(user_id: UserIdDTO, game_id: GameIdDTO) -> Result<UserGameDTO> {
    use server::domain::{GameId, UserId};
    use server::view::get_game;

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let game = get_game(server_state, game_id).await?;
    let game = game.map_or(UserGameDTO::default(), |game| {
        UserGameDTO::from((user_id, &game))
    });

    Ok(game)
}
