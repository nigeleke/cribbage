use crate::{ActiveGameId, GameState, UserId};
use dioxus::prelude::{server_fn::codec::StreamingJson, *};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonStream;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        api_state::ApiState,
        database::{TableChangeEvent, UserGameRow, listen_user_games_changes},
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

fn redis_channel(game_id: &Uuid, user_id: &Uuid) -> String {
    format!("user_game_change::game_{}::user_{}", game_id, user_id)
}

#[server(output = StreamingJson)]
pub async fn user_game_state_stream(
    game_id: ActiveGameId,
    user_id: UserId,
) -> Result<JsonStream<GameState>, ServerFnError> {
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
                                warn!("user_game_state_stream::listener:error - {}", e.to_string());
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
            let event = redis.xread_message::<GameState>(redis_channel(game_id.value(), user_id.value()).as_str()).await?;
            yield Ok(event)
        };
    };

    let stream = stream.filter_map(|res: Result<GameState, ServerFnError>| async move {
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
fn transform_table_change_to_event(
    change: TableChangeEvent<UserGameRow>,
) -> Result<(Uuid, Uuid, GameState), ServiceError> {
    if change.table != "user_games" {
        return Err(ServiceError::InvalidTable(change.table));
    }

    match change.operation.as_str() {
        "UPDATE" => {
            let model = change
                .new_row
                .ok_or(ServiceError::MissingField("new_row".into()))?;
            let game_id = model.game_id;
            let user_id = model.user_id;
            let game_dto = GameState::try_from(model, UserId::from(user_id))?;
            Ok((game_id, user_id, game_dto))
        }
        _ => Err(ServiceError::UnexpectedOperation(change.operation)),
    }
}
