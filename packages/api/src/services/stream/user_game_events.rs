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
        GameEvent::ComputerGameStarted { .. } => {
            warn!("Unhandled GameEvent: {event:?}");
            None
        }
        GameEvent::CutForDealMade { .. } => None,
        GameEvent::CutForDealDecided { .. } => Some(GameEventDTO::CutForDealDecided),
        GameEvent::CutForDealTied => Some(GameEventDTO::CutForDealTied),
        _ => {
            warn!("Unhandled GameEvent: {event:?}");
            None
        }
    };

    let server_state = server_state.clone();
    let stream = server::stream::game_events(server_state, game_id)
        .await
        .map_err(ServerFnError::new)?;

    let mut stream =
        Box::pin(stream.filter_map(move |event| async move { game_event_to_dto(event) }));

    Ok(Streaming::spawn(|tx| async move {
        while let Some(event) = stream.next().await {
            if tx.unbounded_send(event).is_err() {
                break;
            }
        }
    }))
}
