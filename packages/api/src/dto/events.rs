use serde::{Deserialize, Serialize};

use crate::dto::GameIdDTO;

/// Represents a change in the availability of a game.
///
/// This DTO is sent to clients to notify them when a game becomes available
/// or is removed from the list of joinable games.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailableGameEventDTO {
    /// A game has been created and is available for joining.
    Created {
        /// The unique identifier of the game.
        game_id: GameIdDTO,

        /// The display name of the game.
        name: String,
    },
}
