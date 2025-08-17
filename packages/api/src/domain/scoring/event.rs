use crate::{Card, Points, ScoreKind};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    kind: ScoreKind,
    cards: Vec<Card>,
    points: Points,
}

impl Event {
    pub fn new(kind: ScoreKind, cards: Vec<Card>, points: Points) -> Self {
        Self {
            kind,
            cards,
            points,
        }
    }

    pub fn points(&self) -> Points {
        self.points
    }
}
