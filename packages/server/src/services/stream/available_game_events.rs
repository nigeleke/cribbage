use futures::{Stream, TryStreamExt};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::{
    bug,
    convertors::game_query_row_to_game,
    database::{Change, GameQueryRow, Notification},
    domain::{Game, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
};

pub enum AvailableGameEvent {
    Created { game_id: GameId, name: String },
    Removed { game_id: GameId, name: String },
}

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
            let user_can_join = |game: &Game| game.host() != &user_id && game.guest().is_none();
            let player = |game: &Game| game.validate_user(user_id).is_some();
            let joined = |old_game: &Game, new_game: &Game| !player(old_game) && player(new_game);

            let created = |game: &Game| {
                Some(AvailableGameEvent::Created {
                    game_id: *game.id(),
                    name: game.name().clone(),
                })
            };

            let removed = |game: &Game| {
                Some(AvailableGameEvent::Removed {
                    game_id: *game.id(),
                    name: game.name().clone(),
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
        let event = change.map(game_change_to_event).flatten();
        Ok(event)
    });

    let stream = stream.filter_map(|result| match result {
        Ok(event) => Some(event),
        Err(error) => {
            dioxus::prelude::warn!("server:services:available_game_events: error: {error}");
            None
        }
    });

    Ok(stream)
}
