use crate::domain::{Card, Points};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReasonType {
    Fifteen,
    Pair,
    Run,
    Flush,
    HisHeels,
    EndOfPlay,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub const fn points(&self) -> Points {
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
