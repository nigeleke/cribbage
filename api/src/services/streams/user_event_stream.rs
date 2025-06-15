use crate::{UserId, dto::ActiveGame};
use dioxus::prelude::{server_fn::codec::StreamingJson, *};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonStream;
use std::sync::Arc;

#[cfg(feature = "server")]
mod server {
    pub use crate::{
        api_state::ApiState,
        database::{ActiveGameRow, TableChangeEvent},
        services::{
            error::ServiceError,
            listen_and_publish,
            redis::{XReadEvent, user_channel},
        },
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
pub enum UserEvent {
    NewActiveGame(ActiveGame),
    RemovedActiveGame(ActiveGame),
}

#[server(output = StreamingJson)]
pub async fn user_event_stream(user_id: UserId) -> Result<JsonStream<UserEvent>, ServerFnError> {
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
                            "active_games",
                            &pool,
                            transform_to_events,
                            &mut redis,
                        )
                        .await
                        {
                            Ok(game) => {
                                println!("****** active_games_stream: {:?}", game);
                                break;
                            }
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
        let mut redis = redis.clone();
        loop {
            let event = redis.xread_message::<UserEvent>(&user_channel(&user_id)).await?;
            println!("stream BANG loop {:?}", event);
            yield Ok(event)
        };
    };

    let stream = stream.filter_map(|res: Result<UserEvent, ServerFnError>| async move {
        match res {
            Ok(event) => {
                println!("stream filter_map loop {:?}", event);
                Some(event)
            }
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
    change: TableChangeEvent<ActiveGameRow>,
) -> Result<Vec<(String, UserEvent)>, ServiceError> {
    match change {
        TableChangeEvent::InsertAfter {
            table_name: _,
            new_row,
        } => {
            let channel1 = user_channel(UserId::from(new_row.user_id1));
            let channel2 = user_channel(UserId::from(new_row.user_id2));
            let event = UserEvent::NewActiveGame(ActiveGame::from(new_row));
            Ok(vec![(channel1, event.clone()), (channel2, event)])
        }
        TableChangeEvent::DeleteAfter {
            table_name: _,
            old_row,
        } => {
            let channel1 = user_channel(UserId::from(old_row.user_id1));
            let channel2 = user_channel(UserId::from(old_row.user_id2));
            let event = UserEvent::RemovedActiveGame(ActiveGame::from(old_row));
            Ok(vec![(channel1, event.clone()), (channel2, event)])
        }
        other => Err(ServiceError::UnexpectedOperation(other.to_string())),
    }
}
