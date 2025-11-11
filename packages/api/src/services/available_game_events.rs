use crate::{GameEventDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use futures::StreamExt;

#[get("/api/{user_id}/available_game/events", State(server_state): State<server::ServerState>)]
pub async fn available_game_events(
    user_id: UserIdDTO,
) -> Result<Streaming<GameEventDTO, JsonEncoding>, ServerFnError> {
    use server::AvailableGameEvent;

    let user_id = server::UserId::from(user_id.value());

    let game_event_to_dto = |event| match event {
        AvailableGameEvent::Created { .. } => None::<GameEventDTO>,
        AvailableGameEvent::Removed { .. } => None,
    };

    let server_state = server_state.clone();
    let stream = server::available_game_events(server_state, user_id)
        .await
        .map_err(ServerFnError::new)?;

    let stream = stream.filter_map(move |event| async move { game_event_to_dto(event) });
    Ok(Streaming::new(stream))
}
