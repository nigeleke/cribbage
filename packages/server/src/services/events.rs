use cqrs_es::Aggregate;
use cqrs_es::persist::PersistedEventRepository;
use dioxus::prelude::*;
use futures::{Stream, StreamExt, stream};
use postgres_es::PostgresEventRepository;
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    database::{EventRow, Notification},
    error::ServerError,
    server_state::ServerState,
    services::AggregateId,
};

pub async fn events<T>(
    server_state: ServerState,
    aggregate_id: Option<AggregateId>,
) -> Result<impl Stream<Item = (String, T::Event)>, ServerError>
where
    T: Aggregate,
{
    debug!(">>> server:services:events");
    let past_events = past_events::<T>(server_state.clone(), aggregate_id.clone()).await?;
    let future_events = future_events::<T>(server_state.clone(), aggregate_id.clone()).await?;

    Ok(past_events.chain(future_events))
}

async fn past_events<T>(
    server_state: ServerState,
    aggregate_id: Option<AggregateId>,
) -> Result<impl Stream<Item = (AggregateId, T::Event)>, ServerError>
where
    T: Aggregate,
{
    debug!(">>> server:services:past_events");
    let server_state = server_state.clone();
    let pool = server_state.pool.clone();

    let events = PostgresEventRepository::new((*pool).clone());
    let stream = if let Some(aggregate_id) = aggregate_id {
        events.stream_events::<T>(&aggregate_id).await?
    } else {
        events.stream_all_events::<T>().await?
    };

    let stream = stream::unfold(stream, |mut stream| async move {
        match stream.next::<T>(&[]).await {
            Some(Ok(event)) => {
                let aggregate_id = event.aggregate_id;
                let event = event.payload;
                Some(((aggregate_id, event), stream))
            }
            Some(Err(error)) => {
                error!("server:services:past_events error: {error:?}");
                None
            }
            None => None,
        }
    });

    Ok(stream)
}

async fn future_events<T>(
    server_state: ServerState,
    aggregate_id: Option<String>,
) -> Result<impl Stream<Item = (AggregateId, T::Event)>, ServerError>
where
    T: Aggregate,
{
    debug!(">>> server:services:future_events");
    let stream = BroadcastStream::new(server_state.database_changes_sender.subscribe());

    let stream = stream.filter_map(move |result| {
        let aggregate_id = aggregate_id.clone();

        async move {
            debug!("server:services:events:stream.filter_map {result:?}");

            let notification_to_event_row = move |notification: Notification| -> Option<EventRow> {
                debug!("server:services:events:notification_to_event_row {notification:?}");
                if notification.operation == "INSERT" && notification.table_name == "events" {
                    match notification.new_row_as::<EventRow>() {
                        Ok(Some(row)) if aggregate_id.is_none() => Some(row),
                        Ok(Some(row)) if Some(&row.aggregate_id) == aggregate_id.as_ref() => {
                            Some(row)
                        }
                        Ok(Some(_)) => None,
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

            let event_row_to_game_event = |row: EventRow| {
                let aggregate_id = row.aggregate_id.clone();
                let payload = row.payload.clone();
                let event =
                    serde_json::from_value::<T::Event>(payload).map_err(ServerError::from)?;
                Ok::<_, ServerError>(Some((aggregate_id, event)))
            };

            let notification = result.ok()?;
            let row = notification_to_event_row(notification)?;
            event_row_to_game_event(row).ok().flatten()
        }
    });

    Ok(stream)
}
