use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dto::CardIdDTO;

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
    /// Total points scored in this category.
    pub points: usize,

    /// The combinations of cards that contributed to the score.
    pub breakdown: Vec<Vec<CardIdDTO>>,
}

/// Pegging scores broken down by category.
///
/// Maps each `PeggingKindDTO` to a `PeggingSummaryDTO`.
pub type PeggingDTO = HashMap<PeggingKindDTO, PeggingSummaryDTO>;
