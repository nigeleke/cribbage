use crate::{LobbyGameId, dto::StartedGame, services::redis::game_channel};
use dioxus::prelude::{server_fn::codec::StreamingJson, *};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonStream;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        api_state::ApiState,
        database::{StartedGameRow, TableChangeEvent, listen_table_changes},
        services::{error::ServiceError, listen_and_publish, redis::XReadEvent},
        set_no_cache_response,
    };
    pub use async_stream::stream;
    pub use deadpool_redis::Pool;
    pub use dioxus::logger::tracing::warn;
    pub use futures_util::StreamExt;
    pub use std::time::Duration;
    pub use tokio::sync::OnceCell;
    pub static DATABASE_LISTENER: OnceCell<()> = OnceCell::const_new();
}

#[cfg(feature = "server")]
use server::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameEvent {
    NewGame(StartedGame),
}

#[server(output = StreamingJson)]
pub async fn game_event_stream(
    lobby_game_id: LobbyGameId,
) -> Result<JsonStream<GameEvent>, ServerFnError> {
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
                async move {
                    loop {
                        match listen_and_publish(
                            "started_games",
                            &pool,
                            transform_to_events,
                            &mut redis,
                        )
                        .await
                        {
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
        let mut redis = redis.clone();
        loop {
            let event = redis.xread_message::<GameEvent>(&game_channel(&lobby_game_id).as_str()).await?;
            yield Ok(event)
        }
    };

    let stream = stream.filter_map(|res: Result<GameEvent, ServerFnError>| async move {
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
fn transform_to_events(
    change: TableChangeEvent<StartedGameRow>,
) -> Result<Vec<(String, GameEvent)>, ServiceError> {
    match change {
        TableChangeEvent::InsertAfter {
            table_name: _,
            new_row,
        } => {
            let game = StartedGame::from(new_row);
            let channel = game_channel(game.active_game_id());
            let event = GameEvent::NewGame(game);
            Ok(vec![(channel, event)])
        }
        _other => Err(ServiceError::UnexpectedOperation(stringify!(other))),
    }
}
