use dioxus::prelude::*;

use crate::domain::{Card, GameCommand, GameId, UserId};
use crate::server_state::ServerState;
use crate::services::error::ServiceError;
use crate::services::view::get_game;

pub async fn discard_cards_to_crib(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
    cards: Vec<Card>,
) -> Result<(), ServiceError> {
    if let Some(game) = get_game(server_state.clone(), game_id).await? {
        let player = game
            .validate_user(user_id)
            .ok_or(ServiceError::InvalidUser(user_id))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::DiscardCardsToCrib { player, cards };
        let _ = server_state.cqrs.execute(&aggregate_id, command).await?;

        Ok(())
    } else {
        Err(ServiceError::GameNotFound(game_id))
    }
}
