use crate::dto::{GameIdDTO, UserGameDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use futures::StreamExt;

#[get("/api/{user_id}/game/{game_id}/stream", State(server_state): State<server::ServerState>)]
pub async fn user_game_stream(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<Streaming<UserGameDTO, JsonEncoding>, ServerFnError> {
    use crate::convertors;
    use server::domain::{GameId, UserId};
    use server::stream::game_stream;

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let stream = game_stream(server_state, game_id)
        .await
        .map_err(ServerFnError::new)?;

    let stream = stream.filter_map(move |game| async move {
        game.validate_user(user_id)
            .map(|_| convertors::game_to_user_game_dto(&game, &user_id))
    });

    Ok(Streaming::new(stream))
}
