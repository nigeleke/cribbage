use serde::Serialize;

use super::common::Scoring;

#[derive(Debug, Serialize)]
pub struct ScoringPoneType;
pub type ScoringPone = Scoring<ScoringPoneType>;
