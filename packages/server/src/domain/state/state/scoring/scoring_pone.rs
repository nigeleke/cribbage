use serde::{Deserialize, Serialize};

use super::common::Scoring;

/// Marker type used to distinguish pone scoring from other scoring contexts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringPoneType;

/// A strongly typed alias for scoring operations associated with the pone.
///
/// This type helps ensure that computations and values related to pone scoring
/// are handled distinctly within the scoring subsystem.
pub type ScoringPone = Scoring<ScoringPoneType>;
