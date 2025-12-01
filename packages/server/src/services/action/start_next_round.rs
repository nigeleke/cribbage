use crate::{
    bug,
    domain::{GameCommand, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
    services::view::get_game,
};

pub async fn start_next_round(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("score crib".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::StartNextRound { player };

        server_state
            .cqrs
            .execute(&aggregate_id, command)
            .await
            .map_err(bug!())?;

        Ok(())
    } else {
        Err(ServerError::NotFound)
    }
}
