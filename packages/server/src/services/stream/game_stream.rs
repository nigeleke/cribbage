use futures::{Stream, TryStreamExt};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tracing::warn;

use crate::{
    bug, convertors,
    database::{Change, GameQueryRow, Notification},
    domain::{Game, GameId},
    error::ServerError,
    server_state::ServerState,
};

pub async fn game_stream(
    server_state: ServerState,
    game_id: GameId,
) -> Result<impl Stream<Item = Game>, ServerError> {
    let stream =
        BroadcastStream::new(server_state.database_changes_sender.subscribe()).map_err(bug!());

    let stream = stream.try_filter_map(move |notification| async move {
        let notification_to_game_row_change = move |notification: Notification| {
            let change = (notification.table_name == "game_query")
                .then_some(notification.as_change::<GameQueryRow>())
                .transpose()?;
            Ok::<_, ServerError>(change)
        };

        let row_change_to_game_change = move |change: Change<GameQueryRow>| {
            let change = match change {
                Change::Insert { t } => {
                    let t = convertors::game_query_row_to_game(t)?;
                    Change::Insert { t }
                }
                Change::Update { old_t, new_t } => {
                    let old_t = convertors::game_query_row_to_game(old_t)?;
                    let new_t = convertors::game_query_row_to_game(new_t)?;
                    Change::Update { old_t, new_t }
                }
                Change::Delete { t } => {
                    let t = convertors::game_query_row_to_game(t)?;
                    Change::Delete { t }
                }
            };

            Ok::<_, ServerError>(change)
        };

        let game_change_to_game = move |change: Change<Game>| match change {
            Change::Insert { t } if t.id() == &game_id => Some(t),
            Change::Update { new_t, .. } if new_t.id() == &game_id => Some(new_t),
            _ => None,
        };

        let change = notification_to_game_row_change(notification)?;
        let change = change.map(row_change_to_game_change).transpose()?;
        let game = change.and_then(game_change_to_game);

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
