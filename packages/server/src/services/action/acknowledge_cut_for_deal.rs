use crate::domain::{GameCommand, GameId, UserId};
use crate::error::ServerError;
use crate::server_state::ServerState;
use crate::services::view::get_game;

pub async fn acknowledge_cut_for_deal(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("acknowledge_cut_for_deal".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::AcknowledgeCutForDeal { player };
        let _ = server_state
            .cqrs
            .execute(&aggregate_id, command)
            .await
            .map_err(ServerError::bug)?;

        Ok(())
    } else {
        Err(ServerError::NotFound)
    }
}
