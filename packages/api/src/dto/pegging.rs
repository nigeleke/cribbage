use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dto::{CardIdDTO, PlayerDTO};

/// Enumerates the types of pegging scores in a Cribbage game.
///
/// This is used to classify the different scoring categories when tallying points
/// during the pegging phase.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PeggingKindDTO {
    #[doc(hidden)]
    Fifteens,

    #[doc(hidden)]
    Pairs,

    #[doc(hidden)]
    Runs,

    #[doc(hidden)]
    Flush,

    #[doc(hidden)]
    LastCard,

    #[doc(hidden)]
    ThirtyOne,

    #[doc(hidden)]
    HisHeels,

    #[doc(hidden)]
    Nobs,
}

impl std::fmt::Display for PeggingKindDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeggingKindDTO::Fifteens => "Fifteens",
            PeggingKindDTO::Pairs => "Pairs",
            PeggingKindDTO::Runs => "Runs",
            PeggingKindDTO::Flush => "Flush",
            PeggingKindDTO::LastCard => "Last Card",
            PeggingKindDTO::ThirtyOne => "Thirty One",
            PeggingKindDTO::HisHeels => "His Heels",
            PeggingKindDTO::Nobs => "Nobs",
        }
        .fmt(f)
    }
}

/// A summary of pegging points for a particular category.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeggingSummaryDTO {
    /// Total points scored.
    pub points: usize,

    /// The combinations of cards that contributed to the score.
    pub breakdown: Vec<Vec<CardIdDTO>>,
}

/// Mapping from pegging kinds to the collections of cards that made up were pegged.
pub type PeggingBreakdownDTO = HashMap<PeggingKindDTO, PeggingSummaryDTO>;

/// Pegging scores broken down by category.
///
/// Maps each `PeggingKindDTO` to a `PeggingSummaryDTO`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeggingDTO {
    /// Recipient of points scored.
    pub recipient: Option<PlayerDTO>,

    /// Breakdown of the points by kind.
    pub breakdown: PeggingBreakdownDTO,
}

impl PeggingDTO {
    /// Create a new pegging for a receipient
    pub fn new(recipient: PlayerDTO) -> Self {
        Self {
            recipient: Some(recipient),
            breakdown: HashMap::default(),
        }
    }
}
