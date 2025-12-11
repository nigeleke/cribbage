use serde::{Deserialize, Serialize};

use crate::dto::GameIdDTO;

/// Represents the visibility of a game.
///
/// - `Private`: The game is only visible to participating players.
/// - `Public`: The game is visible to all users and can be joined.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailabilityDTO {
    /// The game is private and only accessible to participating players.
    Private,

    /// The game is visible to all users and can be joined.
    Public,
}

/// Games available for the end user to play.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableGameDTO {
    /// Game identifier sortable by creation time.
    pub game_id: GameIdDTO,

    /// A human-readable name for the game, such as "liberated-happy-donkey".
    pub name: String,

    /// The availabiliy to the end user
    pub availability: AvailabilityDTO,
}
