use futures::{Stream, TryStreamExt};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tracing::warn;

use crate::{
    convertors::game_query_row_to_game,
    database::{Change, GameQueryRow, Notification},
    domain::{Game, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
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

    /// An existing game was removed from availability, i.e. someone else joined.
    Removed {
        /// The game idenitifier of the game that was removed.
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
///
/// # Example
///
/// ```no_run
/// use futures::StreamExt;
///
/// let mut stream = available_game_events(server_state.clone(), user_id).await.unwrap();
/// while let Some(event) = stream.next().await {
///     match event {
///         AvailableGameEvent::Created { game_id, name } => { /* handle creation */ },
///         AvailableGameEvent::Removed { game_id, name } => { /* handle removal */ },
///     }
/// }
/// ```
pub async fn available_game_events(
    server_state: ServerState,
    user_id: UserId,
) -> Result<impl Stream<Item = AvailableGameEvent>, ServerError> {
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
                    let t = game_query_row_to_game(t)?;
                    Change::Insert { t }
                }
                Change::Update { old_t, new_t } => {
                    let old_t = game_query_row_to_game(old_t)?;
                    let new_t = game_query_row_to_game(new_t)?;
                    Change::Update { old_t, new_t }
                }
                Change::Delete { t } => {
                    let t = game_query_row_to_game(t)?;
                    Change::Delete { t }
                }
            };
            Ok::<_, ServerError>(change)
        };

        let game_change_to_event = move |change: Change<Game>| {
            let user_can_join = |game: &Game| game.host() != user_id && game.guest().is_none();
            let player = |game: &Game| game.validate_user(user_id).is_some();
            let joined = |old_game: &Game, new_game: &Game| !player(old_game) && player(new_game);

            let created = |game: &Game| {
                Some(AvailableGameEvent::Created {
                    game_id: game.id(),
                    name: String::from(game.name()),
                })
            };

            let removed = |game: &Game| {
                Some(AvailableGameEvent::Removed {
                    game_id: game.id(),
                    name: String::from(game.name()),
                })
            };

            match &change {
                Change::Insert { t } if user_can_join(t) => created(t),
                Change::Update { old_t, new_t } if !joined(old_t, new_t) => removed(new_t),
                Change::Delete { t } if player(t) => removed(t),
                _ => None,
            }
        };

        let change = notification_to_game_row_change(notification)?;
        let change = change.map(row_change_to_game_change).transpose()?;
        let event = change.and_then(game_change_to_event);
        Ok(event)
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
