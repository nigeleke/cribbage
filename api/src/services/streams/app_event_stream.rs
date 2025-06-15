use crate::dto::LobbyGame;
use dioxus::prelude::{server_fn::codec::StreamingJson, *};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonStream;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        api_state::ApiState,
        database::{LobbyGameRow, TableChangeEvent},
        services::{error::ServiceError, redis::app_channel, streams::listen_and_publish},
        set_no_cache_response,
    };
    pub use async_stream::stream;
    pub use dioxus::logger::tracing::warn;
    pub use futures_util::StreamExt;
    pub use std::time::Duration;
    pub use tokio::sync::OnceCell;
    pub static DATABASE_LISTENER: OnceCell<()> = OnceCell::const_new();
}

#[cfg(feature = "server")]
use server::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppEvent {
    NewLobbyGame(LobbyGame),
    RemovedLobbyGame(LobbyGame),
}

#[server(output = StreamingJson)]
pub async fn app_event_stream() -> Result<JsonStream<AppEvent>, ServerFnError> {
    use crate::services::redis::XReadEvent;

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
                let mut redis = redis.clone();
                async move {
                    loop {
                        match listen_and_publish(
                            "lobby_games",
                            &pool,
                            transform_to_events,
                            &mut redis,
                        )
                        .await
                        {
                            Ok(_) => break,
                            Err(e) => {
                                warn!("lobby_games_stream::listener:error - {}", e.to_string());
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
fn transform_to_events(
    change: TableChangeEvent<LobbyGameRow>,
) -> Result<Vec<(String, AppEvent)>, ServiceError> {
    let event = match change {
        TableChangeEvent::InsertAfter {
            table_name: _,
            new_row,
        } => {
            let game = LobbyGame::from(new_row);
            let event = AppEvent::NewLobbyGame(game);
            Ok((app_channel(), event))
        }
        TableChangeEvent::DeleteAfter {
            table_name: _,
            old_row,
        } => {
            let game = LobbyGame::from(old_row);
            let event = AppEvent::RemovedLobbyGame(game);
            Ok((app_channel(), event))
        }
        other => Err(ServiceError::UnexpectedOperation(other.to_string())),
    }?;
    Ok(vec![event])
}
