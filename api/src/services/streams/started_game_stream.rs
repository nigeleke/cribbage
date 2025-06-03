use crate::{UnstartedGameId, dto::StartedGame};
use dioxus::prelude::{server_fn::codec::StreamingJson, *};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonStream;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        api_state::ApiState,
        database::{StartedGameRow, TableChangeEvent, listen_started_games_changes},
        services::{
            error::ServiceError,
            redis::{XAddEvent, XReadEvent},
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

fn redis_channel(id: UnstartedGameId) -> String {
    format!("started_game_change::unstarted_game_{}", id)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    NewGame(StartedGame),
}

#[server(output = StreamingJson)]
pub async fn started_game_stream(
    unstarted_game_id: UnstartedGameId,
) -> Result<JsonStream<Event>, ServerFnError> {
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
                                warn!("started_games_stream::listener:error - {}", e.to_string());
                                tokio::time::sleep(Duration::from_secs(5)).await;
                            }
                        }
                    }
                }
            });
        })
        .await;

    let stream = stream! {
        let mut redis = redis
            .get_connection_manager()
            .await
            .expect("redis connection");
        loop {
            let event = redis.xread_message::<Event>(redis_channel(unstarted_game_id).as_str()).await?;
            yield Ok(event)
        }
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
    use futures_util::StreamExt;

    let table_change_stream = listen_started_games_changes(pool).await?;
    tokio::pin!(table_change_stream);

    while let Some(result) = table_change_stream.next().await {
        let change = result?;
        let (id, event) = transform_table_change_to_event(change)?;

        let _ = redis
            .xadd_message(redis_channel(id).as_str(), &event)
            .await?;
    }

    Ok(())
}

#[cfg(feature = "server")]
fn transform_table_change_to_event(
    change: TableChangeEvent<StartedGameRow>,
) -> Result<(UnstartedGameId, Event), ServiceError> {
    if change.table != "started_games" {
        return Err(ServiceError::InvalidTable(change.table));
    }

    match change.operation.as_str() {
        "INSERT" => {
            let model = change
                .new_row
                .ok_or(ServiceError::MissingField("new_row".into()))?;
            let game = StartedGame::from(model);
            Ok((game.unstarted_game_id, Event::NewGame(game)))
        }
        _ => Err(ServiceError::UnexpectedOperation(change.operation)),
    }
}
