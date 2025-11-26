use std::str::FromStr;

use crate::{
    bug,
    database::{AvailableGameRow, GameQueryRow},
    domain::{Availability, AvailableGame, Game, GameId},
    error::ServerError,
};

pub fn available_game_row_to_available_game(
    row: AvailableGameRow,
) -> Result<AvailableGame, ServerError> {
    let id = GameId::from(row.id);
    let name = row.name;
    let availability = Availability::from_str(&row.availability).map_err(bug!())?;

    let game = AvailableGame::new(id, name, availability);

    Ok(game)
}

#[inline]
pub fn game_query_row_to_game(row: GameQueryRow) -> Result<Game, ServerError> {
    let game = serde_json::from_value::<Game>(row.payload).map_err(bug!())?;
    Ok(game)
}
