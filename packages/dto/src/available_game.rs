use serde::{Deserialize, Serialize};

use crate::GameIdDTO;

/// Games available for the end user to play.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailableGameDTO {
    /// Lobby games created by other users.
    Lobby {
        /// Game identifier sortable by creation time.
        game_id: GameIdDTO,

        /// A human-readable name for the game, such as "liberated-happy-donkey".
        name: String,
    },

    /// Active games created by them.
    Active {
        /// Game identifier sortable by creation time.
        game_id: GameIdDTO,

        /// A human-readable name for the game, such as "liberated-happy-donkey".
        name: String,
    },
}
