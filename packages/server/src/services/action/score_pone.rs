use crate::{
    bug,
    domain::{GameCommand, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
    services::view::get_game,
};

pub async fn score_pone(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
) -> Result<(), ServerError> {
    let game = get_game(server_state.clone(), game_id).await?;

    if let Some(game) = game {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("score pone".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::ScorePone { player };

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
