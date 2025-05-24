use serde::Serialize;

use super::common::Scoring;

#[derive(Debug, Serialize)]
pub struct ScoringCribType;
pub type ScoringCrib = Scoring<ScoringCribType>;
