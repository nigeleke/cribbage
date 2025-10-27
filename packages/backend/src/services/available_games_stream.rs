use dioxus::prelude::*;
use tokio::sync::*;

use crate::GamesStreamEvent;
use crate::domain::{AvailableGame, UserId};
use crate::error::BackendError;
use crate::services::{convertors, games_stream};

#[derive(Clone, Debug)]
pub enum Event {
    Added(AvailableGame),
    Removed(AvailableGame),
}

pub async fn available_games_stream(
    user_id: UserId,
) -> Result<broadcast::Receiver<Event>, BackendError> {
    let mut stream = games_stream().await?;

    let (tx, rx) = broadcast::channel::<Event>(10);

    tokio::spawn(async move {
        while let Ok(event) = stream.recv().await {
            let event = match event {
                GamesStreamEvent::Inserted(game) if game.available_to_user(&user_id) => {
                    let game = convertors::game_to_available_game(&game, &user_id);
                    Some(Event::Added(game))
                }
                GamesStreamEvent::Updated { old_game, new_game }
                    if !old_game.available_to_user(&user_id)
                        && new_game.available_to_user(&user_id) =>
                {
                    let game = convertors::game_to_available_game(&new_game, &user_id);
                    Some(Event::Added(game))
                }
                GamesStreamEvent::Updated { old_game, new_game }
                    if old_game.available_to_user(&user_id)
                        && !new_game.available_to_user(&user_id) =>
                {
                    let game = convertors::game_to_available_game(&old_game, &user_id);
                    Some(Event::Removed(game))
                }
                GamesStreamEvent::Deleted(game) if game.available_to_user(&user_id) => {
                    let game = convertors::game_to_available_game(&game, &user_id);
                    Some(Event::Removed(game))
                }
                _ => None,
            };
            if let Some(event) = event {
                let _ = tx.send(event);
            }
        }
    });

    Ok(rx)
}
