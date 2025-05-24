use serde::Serialize;

use super::common::Scoring;

#[derive(Debug, Serialize)]
pub struct ScoringDealerType;
pub type ScoringDealer = Scoring<ScoringDealerType>;
