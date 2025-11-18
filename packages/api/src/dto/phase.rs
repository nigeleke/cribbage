use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseDTO {
    #[default]
    InLobby,
    CuttingForDeal,
    Discarding,
    Playing,
    ScoringPone,
    ScoringDealer,
    ScoringCrib,
    Finished,
}
