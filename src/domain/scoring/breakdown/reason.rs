use crate::domain::Card;
use crate::domain::Points;

use serde::{Deserialize, Serialize};

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
pub struct Reason {
    typ: ReasonType,
    cards: Vec<Card>,
    points: Points,
}

impl Reason {
    pub fn new(reason_type: ReasonType, cards: &[Card], points: Points) -> Self {
        Self {
            typ: reason_type,
            cards: Vec::from(cards),
            points,
        }
    }

    pub fn points(&self) -> Points {
        self.points
    }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self.cards.iter().map(|c| c.to_string()).collect::<Vec<_>>();
        write!(
            f,
            "{:?}: [{}] => {}",
            self.typ,
            cards.join(", "),
            self.points
        )
    }
}
