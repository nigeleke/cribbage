use dioxus::prelude::*;
use tokio::sync::*;

use crate::database::GameRow;
use crate::domain::{Game, GameId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

pub async fn game_stream(game_id: GameId) -> Result<mpsc::UnboundedReceiver<Game>, BackendError> {
    let mut db_changes = SERVER_STATE.subscribe_database_changes();

    let (tx, rx) = mpsc::unbounded_channel::<Game>();

    tokio::spawn(async move {
        while let Ok(notification) = db_changes.recv().await {
            if notification.table_name == "games"
                && let Ok(Some(row)) = notification.new_row_as::<GameRow>()
                && let Ok(game) = convertors::game_row_to_game(row)
                && game.id() == &game_id
            {
                debug!("backend::game_stream::send {game:?}");
                let _ = tx.send(game);
            } else {
                error!("Failed to convert JSON to Game");
                continue;
            }
        }
    });

    Ok(rx)
}
