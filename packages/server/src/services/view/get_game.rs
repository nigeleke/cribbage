use crate::{
    bug,
    domain::{Game, GameId},
    error::ServerError,
    server_state::ServerState,
};

pub async fn get_game(
    server_state: ServerState,
    game_id: GameId,
) -> Result<Option<Game>, ServerError> {
    use cqrs_es::persist::ViewRepository;

    let game = server_state
        .game_view_repo
        .load(&game_id.value().to_string())
        .await
        .map_err(bug!())?;

    let game = game.map(|g| g.instance().clone());

    Ok(game)
}
