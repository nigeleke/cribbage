use serde::{Deserialize, Serialize};

use crate::domain::{Player, ScoreSheet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pegging {
    player: Player,
    sheet: ScoreSheet,
}

pub trait HasPegging {
    fn pegging(&self) -> &Pegging;
}

impl Pegging {
    pub fn new(player: Player, sheet: ScoreSheet) -> Self {
        Self { player, sheet }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn score_sheet(&self) -> &ScoreSheet {
        &self.sheet
    }
}

impl std::fmt::Display for Pegging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.player, self.sheet)
    }
}
