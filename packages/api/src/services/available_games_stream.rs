use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use dto::{AvailableGameDTO, UserIdDTO};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    Added(AvailableGameDTO),
    Removed(AvailableGameDTO),
}

#[get("/api/{user_id}/available_games/stream")]
pub async fn available_games_stream(
    user_id: UserIdDTO,
) -> Result<Streaming<Event, JsonEncoding>, ServerFnError> {
    use backend::AvailableGamesStreamEvent;

    use crate::services::convertors;

    let user_id = backend::UserId::from(user_id.value());

    let stream = backend::available_games_stream(user_id)
        .await
        .map_err(ServerFnError::new)?;

    let stream = futures::stream::unfold(stream, |mut stream| async move {
        match stream.recv().await {
            Ok(event) => {
                let event = match event {
                    AvailableGamesStreamEvent::Added(game) => {
                        let game = convertors::available_game_to_dto(&game);
                        Event::Added(game)
                    }
                    AvailableGamesStreamEvent::Removed(game) => {
                        let game = convertors::available_game_to_dto(&game);
                        Event::Removed(game)
                    }
                };
                Some((event, stream))
            }
            Err(e) => {
                error!("available_games_stream closed");
                None
            }
        }
    });

    Ok(Streaming::new(stream))
}
