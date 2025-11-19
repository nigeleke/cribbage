use crate::convertors::game_row_to_game;
use crate::database::select_game;
use crate::domain::{Game, GameId};

use crate::error::ServerError;
use crate::server_state::ServerState;

pub async fn get_game(
    server_state: ServerState,
    game_id: GameId,
) -> Result<Option<Game>, ServerError> {
    let pool = server_state.pool.clone();

    let game = select_game(&*pool, game_id.value())
        .await
        .map_err(ServerError::bug)?;

    let game = game
        .map(game_row_to_game)
        .transpose()
        .map_err(ServerError::bug)?;

    Ok(game)
}
