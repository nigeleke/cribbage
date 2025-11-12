use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerDTO {
    User,
    Opponent,
}

// TODO: i18n
impl std::fmt::Display for PlayerDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player = match self {
            PlayerDTO::User => "You",
            PlayerDTO::Opponent => "Opponent",
        };
        player.fmt(f)
    }
}
