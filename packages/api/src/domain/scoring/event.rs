use crate::{Card, Points, ScoreKind, display::format_vec};
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

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: ({}) -> {}",
            self.kind.as_ref(),
            format_vec(self.cards.as_slice()),
            self.points
        )
    }
}
