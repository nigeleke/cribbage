use crate::{UserId, dto::ActiveGame};
use dioxus::prelude::{server_fn::codec::StreamingJson, *};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonStream;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        api_state::ApiState,
        database::{ActiveGameRow, TableChangeEvent, listen_active_games_changes},
        services::{
            error::ServiceError,
            redis::{XAddEvent, XReadEvent},
        },
        set_no_cache_response,
    };
    pub use async_stream::stream;
    pub use dioxus::logger::tracing::warn;
    pub use futures::StreamExt;
    pub use redis::{AsyncCommands, aio::ConnectionManager};
    pub use sqlx::PgPool;
    pub use std::time::Duration;
    pub use tokio::sync::OnceCell;

    pub static DATABASE_LISTENER: OnceCell<()> = OnceCell::const_new();
}

#[cfg(feature = "server")]
use server::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    NewGame(ActiveGame),
    RemovedGame(ActiveGame),
}

fn redis_channel(user_id: &Uuid) -> String {
    format!("active_games_change::user_{}", user_id)
}

#[server(output = StreamingJson)]
pub async fn active_games_stream(user_id: UserId) -> Result<JsonStream<Event>, ServerFnError> {
    set_no_cache_response!();
    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let pool = context.pool().clone();
    let redis = context.redis().clone();

    self::server::DATABASE_LISTENER
        .get_or_init(|| async {
            tokio::spawn({
                let pool = pool.clone();
                let redis = redis.clone();
                let mut redis = redis
                    .get_connection_manager()
                    .await
                    .expect("redis connection");
                async move {
                    loop {
                        match listen_and_publish(&pool, &mut redis).await {
                            Ok(_) => break,
                            Err(e) => {
                                warn!("active_games_stream::listener:error - {}", e.to_string());
                                tokio::time::sleep(Duration::from_secs(5)).await;
                            }
                        }
                    }
                }
            });
        })
        .await;

    let stream = stream! {
        let mut redis = redis.get_connection_manager().await?;
        loop {
            let event = redis.xread_message::<Event>(redis_channel(user_id.value()).as_str()).await?;
            yield Ok(event)
        };
    };

    let stream = stream.filter_map(|res: Result<Event, ServerFnError>| async move {
        match res {
            Ok(event) => Some(event),
            Err(e) => {
                warn!("stream error: {e}");
                None
            }
        }
    });

    Ok(JsonStream::from(stream))
}

#[cfg(feature = "server")]
async fn listen_and_publish(
    pool: &PgPool,
    redis: &mut ConnectionManager,
) -> Result<(), ServiceError> {
    use futures::StreamExt;

    let table_change_stream = listen_active_games_changes(pool).await?;
    tokio::pin!(table_change_stream);

    while let Some(result) = table_change_stream.next().await {
        let change = result?;
        let (user_id1, user_id2, event) = transform_table_change_to_event(change)?;

        let _ = redis
            .xadd_message(redis_channel(&user_id1).as_str(), &event)
            .await?;
        let _ = redis
            .xadd_message(redis_channel(&user_id2).as_str(), &event)
            .await?;
    }

    Ok(())
}

#[cfg(feature = "server")]
fn transform_table_change_to_event(
    change: TableChangeEvent<ActiveGameRow>,
) -> Result<(Uuid, Uuid, Event), ServiceError> {
    if change.table != "active_games" {
        return Err(ServiceError::InvalidTable(change.table));
    }

    match change.operation.as_str() {
        "INSERT" => {
            let model = change
                .new_row
                .ok_or(ServiceError::MissingField("new_row".into()))?;
            Ok((
                model.user_id1,
                model.user_id2,
                Event::NewGame(ActiveGame::from(model)),
            ))
        }
        "DELETE" => {
            let model = change
                .old_row
                .ok_or(ServiceError::MissingField("old_row".into()))?;
            Ok((
                model.user_id1,
                model.user_id2,
                Event::RemovedGame(ActiveGame::from(model)),
            ))
        }
        _ => Err(ServiceError::InvalidOperation(change.operation)),
    }
}
