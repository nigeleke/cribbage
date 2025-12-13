use futures::{Stream, TryStreamExt};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tracing::warn;

use crate::{
    database::{Operation, Timing},
    domain::{Game, GameId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Streams updates for a specific game.
///
/// This function returns a stream of `Game` objects representing changes
/// to the specified game. Each item in the stream reflects the current
/// state of the game after a change (creation, update, or deletion).
///
/// # Parameters
///
/// - `server_state`: The shared server state, including the database change broadcaster.
/// - `game_id`: The ID of the game to track.
///
/// # Returns
///
/// A `Stream` of `Game`s wrapped in `Result`. Errors may occur due to
/// internal server issues, in which case a `ServerError` is returned.
pub async fn game_stream(
    server_state: ServerState,
    game_id: GameId,
) -> Result<impl Stream<Item = Game>, ServerError> {
    let stream =
        BroadcastStream::new(server_state.database_changes_sender.subscribe()).map_err(bug!());

    let stream = stream.try_filter_map(move |notification| {
        let server_state = server_state.clone();

        async move {
            let is_game_query = notification.table_name == "game_query";
            let is_after = matches!(notification.timing, Timing::After);
            let is_upsert = !matches!(notification.operation, Operation::Delete);
            let matches_game = notification
                .primary_key
                .iter()
                .find(|pk| pk.column == "view_id")
                .and_then(|pk| pk.value.as_str())
                .map(|v| v == game_id.value().to_string())
                .unwrap_or(false);

            let should_emit = is_game_query && is_after && is_upsert && matches_game;

            let game = if should_emit {
                get_game(server_state, game_id).await?
            } else {
                None
            };

            Ok(game)
        }
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
