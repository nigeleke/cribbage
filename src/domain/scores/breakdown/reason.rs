use crate::domain::Card;
use crate::types::{Points, HasPoints};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ReasonType {
    Fifteen,
    Pair,
    Run,
    Flush,
    HisHeels,
    EndOfPlay,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Reason(ReasonType, Vec<Card>, Points);

impl Reason {
    pub fn new(reason_type: ReasonType, cards: &[Card], points: Points) -> Self {
        Self(reason_type, Vec::from(cards), points)
    }
}

impl HasPoints for Reason {
    fn points(&self) -> Points {
        self.2
    }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self.1.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        write!(f, "{:?}: [{}] => {}", self.0, cards.join(", "), self.2)
    }
}