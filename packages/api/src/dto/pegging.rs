use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dto::CardIdDTO;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PeggingKindDTO {
    Fifteens,
    Pairs,
    Runs,
    Flush,
    Nob,
}

impl std::fmt::Display for PeggingKindDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeggingKindDTO::Fifteens => "Fifteens",
            PeggingKindDTO::Pairs => "Pairs",
            PeggingKindDTO::Runs => "Runs",
            PeggingKindDTO::Flush => "Flush",
            PeggingKindDTO::Nob => "Nob",
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeggingSummaryDTO {
    pub points: usize,
    pub breakdown: Vec<Vec<CardIdDTO>>,
}

pub type PeggingDTO = HashMap<PeggingKindDTO, PeggingSummaryDTO>;
