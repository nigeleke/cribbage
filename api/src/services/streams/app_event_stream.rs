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
        services::{
            error::ServiceError,
            redis::{XAddEvent, XReadEvent, app_channel},
        },
        set_no_cache_response,
    };
    pub use async_stream::stream;
    pub use dioxus::logger::tracing::warn;
    pub use futures_util::StreamExt;
    pub use redis::{AsyncCommands, aio::ConnectionManager};
    pub use sqlx::PgPool;
    pub use std::time::Duration;
    pub use tokio::sync::OnceCell;
    pub static DATABASE_LISTENER: OnceCell<()> = OnceCell::const_new();
}

#[cfg(feature = "server")]
use server::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppEvent {
    NewGame(UnstartedGame),
    RemovedGame(UnstartedGame),
}

#[server(output = StreamingJson)]
pub async fn app_event_stream() -> Result<JsonStream<AppEvent>, ServerFnError> {
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
                                warn!("unstarted_games_stream::listener:error - {}", e.to_string());
                                tokio::time::sleep(Duration::from_secs(5)).await;
                            }
                        }
                    }
                }
            });
        })
        .await;

    let stream = stream! {
        let mut redis = redis.get_connection_manager().await.expect("redis connection");
        loop {
            let event = redis.xread_message::<AppEvent>(&app_channel()).await?;
            yield Ok(event)
        }
    };

    let stream = stream.filter_map(|res: Result<AppEvent, ServerFnError>| async move {
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
    use futures_util::StreamExt;

    let table_change_stream = listen_unstarted_games_changes(pool).await?;
    tokio::pin!(table_change_stream);

    while let Some(result) = table_change_stream.next().await {
        let change = result?;
        let event = transform_table_change_to_event(change)?;

        let _ = redis.xadd_message(&app_channel(), &event).await?;
    }

    Ok(())
}

#[cfg(feature = "server")]
fn transform_table_change_to_event(
    change: TableChangeEvent<UnstartedGameRow>,
) -> Result<AppEvent, ServiceError> {
    if change.table != "unstarted_games" {
        return Err(ServiceError::InvalidTable(change.table));
    }

    match change.operation.as_str() {
        "INSERT" => {
            let model = change
                .new_row
                .ok_or(ServiceError::MissingField("new_row".into()))?;
            Ok(AppEvent::NewGame(UnstartedGame::from(model)))
        }
        "DELETE" => {
            let model = change
                .old_row
                .ok_or(ServiceError::MissingField("old_row".into()))?;
            Ok(AppEvent::RemovedGame(UnstartedGame::from(model)))
        }
        _ => Err(ServiceError::UnexpectedOperation(change.operation)),
    }
}
