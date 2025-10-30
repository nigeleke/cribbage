use dioxus::prelude::*;
use tokio::sync::*;

use crate::database::GameRow;
use crate::domain::Game;
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

#[derive(Clone, Debug)]
pub enum Event {
    Inserted(Game),
    Updated { old_game: Game, new_game: Game },
    Deleted(Game),
}

pub async fn games_stream() -> Result<broadcast::Receiver<Event>, BackendError> {
    let mut db_changes = SERVER_STATE.subscribe_database_changes();
    let (tx, rx) = broadcast::channel::<Event>(10);

    tokio::spawn(async move {
        let create_event =
            |old_row: Option<GameRow>, new_row: Option<GameRow>| -> Result<Event, BackendError> {
                let old_game = old_row.map(convertors::game_row_to_game).transpose()?;
                let new_game = new_row.map(convertors::game_row_to_game).transpose()?;
                match (old_game, new_game) {
                    (None, Some(new_game)) => Ok(Event::Inserted(new_game)),
                    (Some(old_game), Some(new_game)) => Ok(Event::Updated { old_game, new_game }),
                    (Some(old_game), None) => Ok(Event::Deleted(old_game)),
                    _ => {
                        error!("database update with no before or after");
                        unreachable!()
                    }
                }
            };

        loop {
            match db_changes.recv().await {
                Ok(notification) => {
                    let old_row = notification.old_row_as::<GameRow>()?;
                    let new_row = notification.new_row_as::<GameRow>()?;
                    let event = create_event(old_row, new_row)?;
                    let _ = tx.send(event);
                }
                Err(e) => {
                    error!("games_stream error: {e}");
                    break;
                }
            }
        }

        dioxus::Ok(())
    });

    Ok(rx)
}
