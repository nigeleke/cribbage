use crate::dto::UnstartedGame;
use dioxus::prelude::{server_fn::codec::StreamingJson, *};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonStream;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        api_state::ApiState,
        database::{TableChangeEvent, UnstartedGameRow, listen_unstarted_games_changes},
        services::error::ServiceError,
        set_default_cache,
    };
    pub use async_stream::stream;
    pub use dioxus::logger::tracing::{error, warn};
    pub use futures::StreamExt;
    pub use redis::{AsyncCommands, aio::ConnectionManager};
    pub use sqlx::PgPool;
    pub use std::time::Duration;
    pub use tokio::sync::OnceCell;
    pub const REDIS_CHANNEL: &str = "unstarted_games_change";
    pub static DATABASE_LISTENER: OnceCell<()> = OnceCell::const_new();
}

#[cfg(feature = "server")]
use server::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    NewGame(UnstartedGame),
    RemovedGame(UnstartedGame),
}

#[server(output = StreamingJson)]
pub async fn unstarted_games_stream() -> Result<JsonStream<Event>, ServerFnError> {
    set_default_cache!();
    let context = server_context()
        .get::<Arc<ApiState>>()
        .expect("server initialised");
    let redis_client = context.redis_client().clone();
    let pool = context.pool().clone();
    let redis = context.redis().clone();

    self::server::DATABASE_LISTENER
        .get_or_init(|| async {
            tokio::spawn({
                let pool = pool.clone();
                let mut redis = redis.clone();
                async move {
                    loop {
                        match listen_and_publish(&pool, &mut redis).await {
                            Ok(_) => break,
                            Err(e) => {
                                warn!("unstarted_games_stream::listener:error - {}", e.to_string());
                                tokio::time::sleep(Duration::from_secs(5)).await;
                            }
                        }
                    }
                }
            });
        })
        .await;

    let mut pubsub = redis_client.get_async_pubsub().await.map_err(|e| {
        error!("unstarted_games_stream::redis:pubsub init failed - {}", e);
        e
    })?;
    pubsub.subscribe(REDIS_CHANNEL).await.map_err(|e| {
        error!(
            "unstarted_games_stream::redis:pubsub subscribe fail - {}",
            e
        );
        e
    })?;

    let stream = stream! {
        let mut on_msg_stream = pubsub.on_message();
        while let Some(msg) = on_msg_stream.next().await {
            match msg.get_payload::<String>() {
                Ok(text) => {
                    match serde_json::from_str::<Event>(&text) {
                        Ok(event) => yield event,
                        Err(e) => {
                            warn!("unstarted_games_stream::json_error {}", e.to_string());
                            continue
                        },
                    }
                },
                Err(e) => {
                    warn!("unstarted_games_stream::payload_error {}", e.to_string());
                    continue
                }
            }
        }
        warn!("unstarted_games_stream::stream closed");
    };

    Ok(JsonStream::from(stream))
}

#[cfg(feature = "server")]
async fn listen_and_publish(
    pool: &PgPool,
    redis: &mut ConnectionManager,
) -> Result<(), ServiceError> {
    use futures::StreamExt;

    let table_change_stream = listen_unstarted_games_changes(pool).await?;
    tokio::pin!(table_change_stream);

    while let Some(result) = table_change_stream.next().await {
        let change = result?;
        let event = transform_table_change_to_event(change)?;
        let payload =
            serde_json::to_string(&event).map_err(|e| ServiceError::JsonError(e.to_string()))?;
        redis.publish(REDIS_CHANNEL, &payload).await?
    }

    Ok(())
}

#[cfg(feature = "server")]
fn transform_table_change_to_event(
    change: TableChangeEvent<UnstartedGameRow>,
) -> Result<Event, ServiceError> {
    if change.table != "unstarted_games" {
        return Err(ServiceError::InvalidTable(change.table));
    }
    match change.operation.as_str() {
        "INSERT" => {
            let model = change
                .new_row
                .ok_or(ServiceError::MissingField("new_row".into()))?;
            Ok(Event::NewGame(UnstartedGame::from(model)))
        }
        "DELETE" => {
            let model = change
                .old_row
                .ok_or(ServiceError::MissingField("old_row".into()))?;
            Ok(Event::RemovedGame(UnstartedGame::from(model)))
        }
        _ => Err(ServiceError::InvalidOperation(change.operation)),
    }
}
