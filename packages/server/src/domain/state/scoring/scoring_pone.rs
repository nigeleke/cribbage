use serde::{Deserialize, Serialize};

use super::common::Scoring;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringPoneType;
pub type ScoringPone = Scoring<ScoringPoneType>;
