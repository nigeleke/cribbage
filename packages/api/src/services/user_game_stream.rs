use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use dto::{GameIdDTO, UserGameDTO, UserIdDTO};

#[get("/api/{user_id}/game/{game_id}/stream")]
pub async fn user_game_stream(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<Streaming<UserGameDTO, JsonEncoding>, ServerFnError> {
    use backend::{GameId, UserId};

    use crate::services::convertors;

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());

    let stream = backend::game_stream(game_id)
        .await
        .map_err(ServerFnError::new)?;

    let stream = futures::stream::unfold(stream, move |mut stream| async move {
        match stream.recv().await {
            Ok(game) => {
                let game = convertors::game_to_user_game_dto(&game, &user_id);
                Some((game, stream))
            }
            Err(e) => {
                error!("api::user_game_stream closed {e}");
                None
            }
        }
    });

    Ok(Streaming::from(stream))
}
