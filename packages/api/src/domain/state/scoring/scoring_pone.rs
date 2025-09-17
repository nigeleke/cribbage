use super::common::Scoring;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringPoneType;
pub type ScoringPone = Scoring<ScoringPoneType>;
