use crate::{
    bug,
    domain::{Card, GameCommand, GameId, UserId},
    error::ServerError,
    server_state::ServerState,
    services::view::get_game,
};

pub async fn play_card(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
    card: Card,
) -> Result<(), ServerError> {
    if let Some(game) = get_game(server_state.clone(), game_id).await? {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("play".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::PlayCard { player, card };
        let _ = server_state
            .cqrs
            .execute(&aggregate_id, command)
            .await
            .map_err(bug!())?;

        Ok(())
    } else {
        Err(ServerError::NotFound)
    }
}
