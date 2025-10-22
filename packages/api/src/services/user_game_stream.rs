use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use dto::{GameIdDTO, UserGameDTO, UserIdDTO};

#[get("/api/{user_id}/game/{game_id}/stream")]
pub async fn user_game_stream(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<Streaming<UserGameDTO, JsonEncoding>, ServerFnError> {
    use backend::{GameId, UserId};
    use futures::StreamExt;

    use crate::services::convertors::game_to_user_game_dto;

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    info!("user_game_stream 1");
    let mut stream = backend::game_stream(game_id)
        .await
        .map_err(ServerFnError::new)?;

    info!("user_game_stream 2");
    let stream = stream.map(move |g| {
        info!("user_game_stream 3");
        game_to_user_game_dto(&g, &user_id)
    });

    info!("user_game_stream 4");
    Ok(Streaming::from(stream))
}
