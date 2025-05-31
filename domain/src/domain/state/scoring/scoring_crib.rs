use super::common::Scoring;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoringCribType;
pub type ScoringCrib = Scoring<ScoringCribType>;
