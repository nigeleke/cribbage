use dioxus::{
    fullstack::{JsonEncoding, Streaming, extract::State},
    prelude::*,
};
use futures::StreamExt;

use crate::dto::{GameIdDTO, UserGameDTO, UserIdDTO};

#[get("/api/{user_id}/game/{game_id}/stream", State(server_state): State<server::ServerState>)]
pub async fn user_game_stream(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<Streaming<UserGameDTO, JsonEncoding>> {
    use server::{
        domain::{GameId, UserId},
        stream::game_stream,
    };

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let stream = game_stream(server_state, game_id).await?;
    let stream = stream.filter_map(move |game| async move {
        debug!("________________ user_game_stream: tick {user_id} {game_id}");
        game.validate_user(user_id)
            .map(|_| UserGameDTO::from((user_id, &game)))
    });

    Ok(Streaming::new(stream))
}
