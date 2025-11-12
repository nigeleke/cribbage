use dioxus::prelude::*;
use futures::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

use crate::convertors;
use crate::database::{GameRow, Notification};
use crate::domain::{Game, GameId};
use crate::server_state::ServerState;
use crate::services::error::ServiceError;

pub async fn game_stream(
    server_state: ServerState,
    game_id: GameId,
) -> Result<impl Stream<Item = Game>, ServiceError> {
    let stream = BroadcastStream::new(server_state.database_changes_sender.subscribe());

    let stream = stream.filter_map(move |result| async move {
        let notification_to_game_row = move |notification: Notification| {
            debug!("server:services:game_stream:notification: {notification:?}");
            if notification.table_name == "games" {
                if notification.operation == "INSERT" || notification.operation == "UPDATE" {
                    match notification.new_row_as::<GameRow>() {
                        Ok(Some(row)) => Some(row),
                        Ok(None) => {
                            error!("internal error: failed to get game: nothing inserted/updated");
                            None
                        }
                        Err(error) => {
                            error!("internal error: failed to get game: {error:?}");
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        let game_row_to_game = |row: GameRow| {
            let game = convertors::game_row_to_game(row).ok()?;
            (game.id() == &game_id).then_some(game)
        };

        let notification = result.ok()?;
        let row = notification_to_game_row(notification)?;
        game_row_to_game(row)
    });

    Ok(stream)
}
