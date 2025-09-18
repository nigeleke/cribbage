use serde::{Deserialize, Serialize};

use super::common::Scoring;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringDealerType;
pub type ScoringDealer = Scoring<ScoringDealerType>;
