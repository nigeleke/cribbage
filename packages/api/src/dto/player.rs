use serde::{Deserialize, Serialize};

/// Represents a player in the game from the client’s perspective.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PlayerDTO {
    /// The current user of the API.
    User,

    /// The opponent player.
    Opponent,
}

impl std::fmt::Display for PlayerDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player = match self {
            PlayerDTO::User => "You",
            PlayerDTO::Opponent => "Opponent",
        };
        player.fmt(f)
    }
}
