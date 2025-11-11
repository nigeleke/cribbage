use dioxus::prelude::*;

use crate::{
    domain::{Game, GameEvent, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
    services::events::events,
};
use futures::{Stream, StreamExt};

pub enum AvailableGameEvent {
    Created { game_id: GameId, name: String },
    Removed { game_id: GameId },
}

pub async fn available_game_events(
    server_state: ServerState,
    user_id: UserId,
) -> Result<impl Stream<Item = AvailableGameEvent>, ServerError> {
    debug!(">>> server:services:available_game_events");
    let stream = events::<Game>(server_state, None).await?;
    let stream = stream.filter_map(|event| async move { None })
        //     let event = match event {
        //         GameEvent::LobbyGameCreated {
        //             game_id,
        //             host,
        //             name,
        //         } if host == user_id => Some(AvailableGameEvent::Created { game_id, name }),
        //         GameEvent::LobbyGameJoined { guest } if guest == user_id => {
        //             Some(AvailableGameEvent::Created { game_id, name })
        //         }
        //         GameEvent::LobbyGameJoined { guest } if guest != user_id => {
        //             Some(AvailableGameEvent::Removed { game_id })
        //         }
        //         GameEvent::ComputerGameStarted {
        //             game_id,
        //             host,
        //             name,
        //             ..
        //         } => Some(AvailableGameEvent::Created { game_id, name }),
        //         _ => None,
        //     };
        //     Ok(event)
        // })
        ;
    Ok(stream)
}
