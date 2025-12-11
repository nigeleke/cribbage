use serde::{Deserialize, Serialize};

use super::common::Scoring;

/// Marker type used to distinguish dealer scoring from other scoring contexts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringDealerType;

/// A strongly typed alias for scoring operations associated with the dealer.
///
/// This type ensures that scoring computations relating to the dealer are
/// handled distinctly from other scoring domains.
pub type ScoringDealer = Scoring<ScoringDealerType>;
