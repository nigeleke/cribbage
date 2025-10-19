use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use dto::{GameIdDTO, UserGameDTO, UserIdDTO};

#[get("/api/{user_id}/stream/game/{game_id}")]
pub async fn user_game_stream(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<Streaming<UserGameDTO, JsonEncoding>, ServerFnError> {
    use backend::{GameId, UserId};
    use futures::StreamExt;

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    dioxus::logger::tracing::info!("user_game_stream 0");

    let rx = backend::game_stream(game_id)
        .await
        .map_err(ServerFnError::new)?;

    let rx = rx.map(|game| {
        dioxus::logger::tracing::info!("user_game_stream 1");
        UserGameDTO::new(game.name())
    });

    Ok(Streaming::new(rx))
}
