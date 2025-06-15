use crate::{
    database::{TableChangeEvent, listen_table_changes},
    services::ServiceError,
};
use deadpool_redis::Pool as RedisPool;
use serde::{Serialize, de::DeserializeOwned};
use sqlx::PgPool;

pub async fn listen_and_publish<T, E>(
    table_name: &str,
    pool: &PgPool,
    change_to_events: fn(TableChangeEvent<T>) -> Result<Vec<(String, E)>, ServiceError>,
    redis: &mut RedisPool,
) -> Result<(), ServiceError>
where
    E: Serialize + std::fmt::Debug,
    T: DeserializeOwned + std::fmt::Debug,
{
    use crate::services::redis::XAddEvent;
    use futures_util::StreamExt;

    let table_change_stream = listen_table_changes(table_name, pool).await?;
    tokio::pin!(table_change_stream);

    while let Some(result) = table_change_stream.next().await {
        use futures_util::{TryStreamExt, stream};

        let change = result?;
        let events = change_to_events(change)?;

        stream::iter(events.into_iter().map(Ok::<_, ServiceError>))
            .try_for_each_concurrent(None, |(stream, event)| {
                let redis = &*redis;
                async move {
                    println!(
                        "_-_-_-_-_ adding event to stream: event {:?}, stream {stream}",
                        event
                    );
                    redis.xadd_message(&stream, &event).await?;
                    Ok(())
                }
            })
            .await?;
    }

    Ok(())
}
