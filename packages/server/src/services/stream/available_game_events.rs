use std::str::FromStr;

use dioxus::prelude::*;

use crate::{
    domain::{Game, GameEvent, GameId, UserId},
    server_state::ServerState,
    services::{error::ServiceError, stream::events::events, view::get_game},
};
use futures::{Stream, StreamExt};

pub enum AvailableGameEvent {
    Created { game_id: GameId, name: String },
    Removed { game_id: GameId },
}

pub async fn available_game_events(
    server_state: ServerState,
    _user_id: UserId,
) -> Result<impl Stream<Item = AvailableGameEvent>, ServiceError> {
    let stream = events::<Game>(server_state.clone(), None).await?;
    let stream = stream.filter_map({
        move |(aggregate_id, event)| {
            let server_state = server_state.clone();
            async move {
                if let Ok(game_id) = GameId::from_str(&aggregate_id)
                    && let Ok(_game) = get_game(server_state, game_id).await
                {
                    match event {
                        GameEvent::LobbyGameCreated { game_id, name, .. } => {
                            Some(AvailableGameEvent::Created { game_id, name })
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    });
    Ok(stream)
}
