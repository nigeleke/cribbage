use super::common::Scoring;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoringDealerType;
pub type ScoringDealer = Scoring<ScoringDealerType>;
