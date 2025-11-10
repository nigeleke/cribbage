use dioxus::prelude::*;
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    convertors,
    database::{EventRow, Notification},
    domain::{Game, GameEvent, GameId},
    error::ServerError,
    server_state::ServerState,
};
use cqrs_es::persist::PersistedEventRepository;
use futures::{Stream, StreamExt, stream};
use postgres_es::PostgresEventRepository;

pub async fn game_events(
    server_state: ServerState,
    game_id: GameId,
) -> Result<impl Stream<Item = GameEvent>, ServerError> {
    debug!(">>> server:services:game_events");
    let aggregate_id = game_id.value().to_string();
    let past_events = past_events(server_state.clone(), aggregate_id.clone()).await?;
    let future_events = future_events(server_state.clone(), aggregate_id.clone()).await?;

    Ok(past_events.chain(future_events))
}

async fn past_events(
    server_state: ServerState,
    aggregate_id: String,
) -> Result<impl Stream<Item = GameEvent>, ServerError> {
    debug!(">>> server:services:past_events");
    let server_state = server_state.clone();
    let pool = server_state.pool.clone();

    let events = PostgresEventRepository::new((*pool).clone());
    let stream = events.stream_events::<Game>(&aggregate_id).await?;

    let stream = stream::unfold(stream, |mut stream| async move {
        match stream.next::<Game>(&[]).await {
            Some(Ok(event)) => {
                let event = event.payload;
                Some((event, stream))
            }
            Some(Err(error)) => {
                error!("server:services:past_events error: {error:?}");
                None
            }
            None => {
                warn!("server:services:past_events error: stream closed");
                None
            }
        }
    });

    Ok(stream)
}

async fn future_events(
    server_state: ServerState,
    aggregate_id: String,
) -> Result<impl Stream<Item = GameEvent>, ServerError> {
    debug!(">>> server:services:future_events");
    let stream = BroadcastStream::new(server_state.database_changes_sender.subscribe());

    let notification_to_event_row = |notification: Notification| -> Option<EventRow> {
        debug!("server:services:game_events:notification_to_event_row {notification:?}");
        if notification.operation == "INSERT" && notification.table_name == "events" {
            match notification.new_row_as::<EventRow>() {
                Ok(Some(row)) => Some(row),
                Ok(None) => {
                    error!("internal error: failed to get event: nothing inserted");
                    None
                }
                Err(error) => {
                    error!("internal error: failed to get event: {error:?}");
                    None
                }
            }
        } else {
            None
        }
    };

    let is_game_aggregate = |aggregate_id: &str, row: &EventRow| {
        debug!("server:services:game_events:is_game_aggregate {aggregate_id:?} {row:?}");
        row.aggregate_id == aggregate_id && row.aggregate_type == "Game"
    };

    let event_row_to_game_event = |row: EventRow| convertors::json_to_game_event(row.payload).ok();

    let stream = stream.filter_map(move |result| {
        let aggregate_id = aggregate_id.clone();
        async move {
            debug!("server:services:game_events:stream.filter_map {result:?}");
            let notification = result.ok()?;
            let row = notification_to_event_row(notification)?;
            if is_game_aggregate(&aggregate_id, &row) {
                event_row_to_game_event(row)
            } else {
                None
            }
        }
    });

    Ok(stream)
}
