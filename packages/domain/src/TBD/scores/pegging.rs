use crate::{Player, ScoreSheet};

/// Represents pegging information for a specific player during scoring.
///
/// Pegging is the process of awarding points during the play
/// sequence but is also used here for pegging of the pone hand, dealer hand
/// and crib.
///
/// This structure associates a player with the score sheet recording their
/// pegging-related points.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pegging {
    recipient: Player,
    sheet: ScoreSheet,
}

impl Pegging {
    /// Constructs a new pegging record for the specified `player`, using the
    /// provided `sheet` as the underlying scoring record.
    ///
    /// Callers are responsible for ensuring that the `sheet` is appropriate
    /// for pegging-scoring usage.
    pub fn new(player: Player, sheet: ScoreSheet) -> Self {
        Self {
            recipient: player,
            sheet,
        }
    }

    /// Returns an immutable reference to the player associated with this pegging record.
    pub fn recipient(&self) -> &Player {
        &self.recipient
    }

    /// Returns an immutable reference to the score sheet that records pegging-related points.
    pub fn score_sheet(&self) -> &ScoreSheet {
        &self.sheet
    }
}

impl std::fmt::Display for Pegging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.recipient, self.sheet)
    }
}
