use std::str::FromStr;

use futures::{Stream, TryStreamExt};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tracing::warn;

use crate::{
    database::{Operation, Timing},
    domain::{GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Represents a change in the list of available games for a user.
pub enum AvailableGameEvent {
    /// A new game was created and is available to join (hosted)
    /// or play (computer).
    Created {
        /// The game idenitifier of the game that was created.
        game_id: GameId,

        /// The game's name.
        name: String,
    },
}

/// Streams updates about games that are available to the given user.
///
/// The stream yields `AvailableGameEvent`s when a game is created or removed
/// from the list of games available to the user.
///
/// # Parameters
///
/// - `server_state`: The shared server state, including the database change broadcaster.
/// - `user_id`: The ID of the user for whom available games are tracked.
///
/// # Returns
///
/// A `Stream` of `AvailableGameEvent`s wrapped in `Result`.
/// Errors may occur due to internal server issues.
pub async fn available_game_events(
    server_state: ServerState,
    user_id: UserId,
) -> Result<impl Stream<Item = AvailableGameEvent>, ServerError> {
    let stream =
        BroadcastStream::new(server_state.database_changes_sender.subscribe()).map_err(bug!());

    let stream = stream.try_filter_map(move |notification| {
        let server_state = server_state.clone();

        async move {
            let is_game_query = notification.table_name == "game_query";
            let is_after = matches!(notification.timing, Timing::After);

            let may_emit = is_game_query && is_after;

            let event = may_emit
                .then_some({
                    let game_id = notification
                        .primary_key
                        .iter()
                        .find(|pk| pk.column == "view_id")
                        .and_then(|pk| pk.value.as_str())
                        .map(GameId::from_str)
                        .transpose()
                        .map_err(bug!())?;

                    if let Some(game_id) = game_id
                        && let Some(game) = get_game(server_state, game_id).await?
                    {
                        let user_can_join = game.host() != user_id && game.guest().is_none();
                        let valid_player = game.validate_user(user_id).is_some();

                        match notification.operation {
                            Operation::Insert if user_can_join || valid_player => {
                                Some(AvailableGameEvent::Created {
                                    game_id: game.id(),
                                    name: String::from(game.name()),
                                })
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .flatten();

            Ok(event)
        }
    });

    let stream = stream.filter_map(|result| match result {
        Ok(event) => Some(event),
        Err(error) => {
            warn!("server:services:available_game_events: error: {error}");
            None
        }
    });

    Ok(stream)
}
