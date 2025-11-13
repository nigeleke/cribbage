use cqrs_es::Aggregate;
use cqrs_es::persist::PersistedEventRepository;
use dioxus::prelude::*;
use futures::{Stream, TryStreamExt};
use postgres_es::PostgresEventRepository;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::{
    database::{Change, EventRow},
    server_state::ServerState,
    services::{AggregateId, error::ServiceError},
};

pub async fn events<T>(
    server_state: ServerState,
    aggregate_id: Option<AggregateId>,
) -> Result<impl Stream<Item = (AggregateId, T::Event)>, ServiceError>
where
    T: Aggregate,
{
    let past_events = past_events::<T>(server_state.clone(), aggregate_id.clone()).await?;
    let future_events = future_events::<T>(server_state.clone(), aggregate_id.clone()).await?;

    Ok(past_events.chain(future_events))
}

async fn past_events<T>(
    server_state: ServerState,
    aggregate_id: Option<AggregateId>,
) -> Result<impl Stream<Item = (AggregateId, T::Event)>, ServiceError>
where
    T: Aggregate,
{
    use futures::stream;

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
                warn!("server:services:past_events error: {error:?}");
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
) -> Result<impl Stream<Item = (AggregateId, T::Event)>, ServiceError>
where
    T: Aggregate,
{
    let stream = BroadcastStream::new(server_state.database_changes_sender.subscribe())
        .map_err(ServiceError::from);

    let stream = stream.try_filter_map(move |notification| {
        let aggregate_id = aggregate_id.clone();

        async move {
            let wanted_aggregate = |t: &EventRow| {
                aggregate_id.is_none() || Some(&t.aggregate_id) == aggregate_id.as_ref()
            };

            let id_event = if notification.table_name == "events" {
                let change = notification.as_change::<EventRow>()?;

                match change {
                    Change::Insert { t } if wanted_aggregate(&t) => {
                        let aggregate_id = t.aggregate_id.clone();
                        let payload = t.payload.clone();
                        let event = serde_json::from_value::<T::Event>(payload)?;
                        Some((aggregate_id, event))
                    }
                    _ => None,
                }
            } else {
                None
            };

            Ok::<_, ServiceError>(id_event)
        }
    });

    let stream = stream.filter_map(|result| match result {
        Ok(id_event) => Some(id_event),
        Err(error) => {
            warn!("server:services:future_events: error: {error}");
            None
        }
    });

    Ok(stream)
}
