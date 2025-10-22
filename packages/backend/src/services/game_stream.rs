use dioxus::prelude::*;
use futures::SinkExt;

use crate::database::GameRow;
use crate::domain::{Game, GameId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

pub async fn game_stream(game_id: GameId) -> Result<UnboundedReceiver<Game>, BackendError> {
    let mut db_changes = SERVER_STATE.subscribe_database_changes();
    let (mut tx, rx) = futures::channel::mpsc::unbounded::<Game>();

    tokio::spawn(async move {
        info!("game_stream 1");
        while let Ok(notification) = db_changes.recv().await {
            info!("game_stream 2");
            if let Ok(Some(row)) = notification.new_row_as::<GameRow>()
                && let Ok(game) = convertors::game_row_to_game(row)
                && game.id() == &game_id
            {
                info!("game_stream 3");
                let _ = tx.send(game);
            } else {
                info!("game_stream error");
                error!("Failed to convert JSON to Game");
                continue;
            }
        }
    });

    Ok(rx)
}
