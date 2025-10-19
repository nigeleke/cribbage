use futures::StreamExt;

use crate::domain::{Game, GameId, UserId};
use crate::error::BackendError;
use crate::name_builder::generate_game_name;
// use crate::{SERVER_STATE, database};

pub async fn game_stream(
    _game_id: GameId,
) -> Result<dioxus::prelude::UnboundedReceiver<Game>, BackendError> {
    let (tx, rx) = futures::channel::mpsc::unbounded();

    tokio::spawn(async move {
        loop {
            //         // TODO:
            //         // let mut game_stream = database::game_change_stream(SERVER_STATE.postgres_pool())
            //         //     .await
            //         //     .expect("must be valid game_change_stream");
            //         // let result = game_stream.next().await;

            //         // dioxus::logger::tracing::info!("GOT STREAM CHANGE RESULT {result:?}");

            //         // let game = result.expect("valid result not got");

            //         // dioxus::logger::tracing::info!("GOT STREAM CHANGE {game:?}");

            let result = tx.unbounded_send(Game::host_game(UserId::new(), generate_game_name()));
            dioxus::logger::tracing::info!("game_stream::result {result:?}");

            if result.is_err() {
                dioxus::logger::tracing::info!("game_stream::result:is_err()");
                break;
            }

            dioxus::logger::tracing::info!("game_stream::result:is_not_err()");
            tokio::time::sleep(tokio::time::Duration::from_millis(6000)).await;
        }
    });

    Ok(rx)
}
