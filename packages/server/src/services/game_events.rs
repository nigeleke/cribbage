use dioxus::prelude::*;

use crate::{
    domain::{Game, GameEvent, GameId},
    error::ServerError,
    server_state::ServerState,
    services::events::events,
};
use futures::{Stream, StreamExt};

pub async fn game_events(
    server_state: ServerState,
    game_id: GameId,
) -> Result<impl Stream<Item = GameEvent>, ServerError> {
    debug!(">>> server:services:game_events");
    let aggregate_id = game_id.value().to_string();
    let stream = events::<Game>(server_state, Some(aggregate_id)).await?;
    Ok(stream.map(|(_, event)| event))
}
