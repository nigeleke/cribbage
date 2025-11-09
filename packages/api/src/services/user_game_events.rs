use crate::{GameEventDTO, GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::*;
use futures::StreamExt;

#[get("/api/{user_id}/game/{game_id}/events", State(server_state): State<server::ServerState>)]
pub async fn user_game_events(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
) -> Result<Streaming<GameEventDTO, JsonEncoding>, ServerFnError> {
    use server::GameEvent;

    let _user_id = server::UserId::from(user_id.value());
    let game_id = server::GameId::from(game_id.value());

    let game_event_to_dto = |event| match event {
        GameEvent::LobbyGameCreated { name, .. } => Some(GameEventDTO::LobbyGameCreated { name }),
        GameEvent::LobbyGameJoined { .. } => Some(GameEventDTO::OpponentJoined),
        _ => None,
    };

    let server_state = server_state.clone();
    let stream = server::game_events(server_state, game_id)
        .await
        .map_err(ServerFnError::new)?;

    let stream = stream.filter_map(move |event| async move { game_event_to_dto(event) });
    Ok(Streaming::new(stream))
}
