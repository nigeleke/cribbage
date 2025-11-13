use dioxus::prelude::*;
use futures::{Stream, TryStreamExt};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::convertors;
use crate::database::{Change, GameRow, Notification};
use crate::domain::{Game, GameId};
use crate::server_state::ServerState;
use crate::services::error::ServiceError;

pub async fn game_stream(
    server_state: ServerState,
    game_id: GameId,
) -> Result<impl Stream<Item = Game>, ServiceError> {
    let stream = BroadcastStream::new(server_state.database_changes_sender.subscribe())
        .map_err(ServiceError::from);

    let stream = stream.try_filter_map(move |notification| async move {
        let game_id = game_id.clone();

        let notification_to_game_row_change = move |notification: Notification| {
            let change = (notification.table_name == "games")
                .then_some(notification.as_change::<GameRow>())
                .transpose()?;
            Ok::<_, ServiceError>(change)
        };

        let row_change_to_game_change = move |change: Change<GameRow>| {
            let change = match change {
                Change::Insert { t } => {
                    let t = convertors::game_row_to_game(t)?;
                    Change::Insert { t }
                }
                Change::Update { old_t, new_t } => {
                    let old_t = convertors::game_row_to_game(old_t)?;
                    let new_t = convertors::game_row_to_game(new_t)?;
                    Change::Update { old_t, new_t }
                }
                Change::Delete { t } => {
                    let t = convertors::game_row_to_game(t)?;
                    Change::Delete { t }
                }
            };
            Ok::<_, ServiceError>(change)
        };

        let game_change_to_game = move |change: Change<Game>| match change {
            Change::Insert { t } if t.id() == &game_id => Some(t),
            Change::Update { new_t, .. } if new_t.id() == &game_id => Some(new_t),
            _ => None,
        };

        let change = notification_to_game_row_change(notification)?;
        let change = change.map(row_change_to_game_change).transpose()?;
        let game = change.map(game_change_to_game).flatten();

        Ok(game)
    });

    let stream = stream.filter_map(|result| match result {
        Ok(game) => Some(game),
        Err(error) => {
            warn!("server:services:game_stream: error: {error}");
            None
        }
    });

    Ok(stream)
}
