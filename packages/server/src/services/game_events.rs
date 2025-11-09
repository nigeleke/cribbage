use dioxus::prelude::*;

use crate::{Game, GameEvent, GameId, ServerError, ServerState};
use cqrs_es::persist::PersistedEventRepository;
use futures::{Stream, StreamExt, stream};
use postgres_es::PostgresEventRepository;

pub async fn game_events(
    server_state: ServerState,
    game_id: GameId,
) -> Result<impl Stream<Item = GameEvent>, ServerError> {
    let aggregate_id = game_id.value().to_string();

    let server_state = server_state.clone();
    let pool = server_state.pool.clone();

    let events = PostgresEventRepository::new((*pool).clone());
    let stream = events.stream_events::<Game>(&aggregate_id).await?;

    let stream = stream::unfold(stream, |mut stream| async move {
        match stream.next::<Game>(&[]).await {
            Some(Ok(event)) => Some((Ok(event.payload), stream)),
            Some(Err(error)) => Some((Err(ServerError::from(error)), stream)),
            None => None,
        }
    });

    Ok(stream
        .take_while(|res| futures::future::ready(res.is_ok()))
        .filter_map(|res| async move { res.ok() }))
}
