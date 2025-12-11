use serde::{Deserialize, Serialize};

use super::{CardDTO, ScoreDTO};

/// Represents the current state of a player for API clients.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerStateDTO {
    /// The card the player cut for the deal, if it exists.
    /// This is only of relevance in the cut_for_deal phase.
    pub cut: Option<CardDTO>,

    /// The cards currently in the player's hand. These will initially be face
    /// down if the state represents the opponent.
    pub hand: Vec<CardDTO>,

    /// The player's current score.
    pub score: ScoreDTO,
}
