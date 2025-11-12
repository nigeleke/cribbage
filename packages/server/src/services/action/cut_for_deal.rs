use dioxus::prelude::*;

use crate::domain::{GameCommand, GameId, UserId};
use crate::server_state::ServerState;
use crate::services::error::ServiceError;
use crate::services::view::get_game;

pub async fn cut_for_deal(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServiceError> {
    if let Some(game) = get_game(server_state.clone(), game_id).await? {
        let player = game
            .validate_user(user_id)
            .ok_or(ServiceError::InvalidUser(user_id))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::CutForDeal { player };
        let _ = server_state.cqrs.execute(&aggregate_id, command).await?;

        Ok(())
    } else {
        Err(ServiceError::GameNotFound(game_id))
    }
}
