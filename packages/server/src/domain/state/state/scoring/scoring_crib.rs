use serde::{Deserialize, Serialize};

use super::common::Scoring;

/// Marker type used to distinguish crib scoring from other scoring contexts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringCribType;

/// A strongly typed alias for scoring operations related to the crib.
///
/// This type ensures that crib scoring cannot be confused with hand
/// scoring or other scoring phases.
pub type ScoringCrib = Scoring<ScoringCribType>;
