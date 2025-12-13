use std::str::FromStr;

use crate::{
    database::AvailableGameRow,
    domain::{Availability, AvailableGame, GameId},
    error::{ServerError, bug},
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
