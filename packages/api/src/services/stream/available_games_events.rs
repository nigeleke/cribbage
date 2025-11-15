use crate::{AvailableGameEventDTO, GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use futures::StreamExt;

#[get("/api/{user_id}/available_games/events", State(server_state): State<server::ServerState>)]
pub async fn available_games_events(
    user_id: UserIdDTO,
) -> Result<Streaming<AvailableGameEventDTO, JsonEncoding>, ServerFnError> {
    use server::stream::AvailableGameEvent;

    let user_id = server::UserId::from(user_id.value());

    let game_event_to_dto = |event| match event {
        AvailableGameEvent::Created { game_id, name } => {
            let game_id = GameIdDTO::from(game_id.value());
            AvailableGameEventDTO::Created { game_id, name }
        }
        AvailableGameEvent::Removed { game_id, name } => {
            let game_id = GameIdDTO::from(game_id.value());
            AvailableGameEventDTO::Removed { game_id, name }
        }
    };

    let stream = server::stream::available_game_events(server_state, user_id)
        .await
        .map_err(ServerFnError::new)?;

    let mut stream = Box::pin(stream.map(move |event| game_event_to_dto(event)));

    Ok(Streaming::spawn(|tx| async move {
        while let Some(event) = stream.next().await {
            if tx.unbounded_send(event).is_err() {
                break;
            }
        }
    }))
}
