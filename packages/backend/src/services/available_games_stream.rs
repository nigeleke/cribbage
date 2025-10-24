use dioxus::logger::tracing::*;
use tokio::sync::*;

use crate::database::GameRow;
use crate::domain::{AvailableGame, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

#[derive(Clone, Debug)]
pub enum Event {
    Added(AvailableGame),
    Removed(AvailableGame),
}

pub async fn available_games_stream(
    user_id: UserId,
) -> Result<mpsc::UnboundedReceiver<Event>, BackendError> {
    let mut db_changes = SERVER_STATE.subscribe_database_changes();
    let (tx, rx) = mpsc::unbounded_channel();

    debug!("backend::available_games_stream:listening: {user_id:?}");

    tokio::spawn(async move {
        let create_event = |old_row: Option<GameRow>,
                            new_row: Option<GameRow>|
         -> Result<Option<Event>, BackendError> {
            debug!("backend::available_games_stream:creating_event:");
            let old_game = old_row.map(convertors::game_row_to_game).transpose()?;
            let new_game = new_row.map(convertors::game_row_to_game).transpose()?;

            let old_game_has_user = old_game
                .as_ref()
                .map(|g| g.has_user(&user_id))
                .unwrap_or(false);
            let new_game_has_user = new_game
                .as_ref()
                .map(|g| g.has_user(&user_id) || g.guest().is_none())
                .unwrap_or(false);

            debug!(
                "backend::available_games_stream:creating_event: old_game_user: {old_game_has_user} new_game_user: {new_game_has_user}"
            );

            let event = match (old_game_has_user, new_game_has_user) {
                (false, false) => None,
                (false, true) => new_game.map(|new_game| {
                    let game = convertors::game_to_available_game(&new_game, &user_id);
                    Event::Added(game)
                }),
                (true, false) => old_game.map(|old_game| {
                    let game = convertors::game_to_available_game(&old_game, &user_id);
                    Event::Removed(game)
                }),
                (true, true) => new_game.map(|new_game| {
                    let game = convertors::game_to_available_game(&new_game, &user_id);
                    Event::Added(game)
                }),
            };

            debug!("backend::available_games_stream:created_event: {event:?}");

            Ok(event)
        };

        loop {
            match db_changes.recv().await {
                Ok(notification) if notification.table_name == "games" => {
                    debug!("backend::available_games_stream:received: {notification:?}");
                    let old_row = notification.old_row_as::<GameRow>()?;
                    let new_row = notification.new_row_as::<GameRow>()?;
                    let event = create_event(old_row, new_row)?;
                    if let Some(event) = event {
                        debug!("backend::available_games_stream::send {event:?}");
                        let _ = tx.send(event);
                    }
                }
                Ok(notification) => {
                    debug!("backend::available_games_stream:received: (IGNORING) {notification:?}");
                }
                Err(e) => {
                    error!("available_games_stream error: {e}");
                    break;
                }
            }
        }

        dioxus::Ok(())
    });

    Ok(rx)
}
