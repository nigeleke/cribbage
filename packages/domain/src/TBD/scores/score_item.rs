use crate::{Card, Points, ScoreKind};

/// Represents a single scoring event in the game.
///
/// Each `ScoreItem` captures:
/// - the type of score (`kind`),
/// - the set of cards that contributed to the score (`cards`),
/// - and the number of points awarded (`points`).
#[derive(Clone, PartialEq, Eq)]
pub struct ScoreItem {
    kind: ScoreKind,
    cards: Vec<Card>,
    points: Points,
}

impl ScoreItem {
    /// Constructs a new `ScoreItem` with the specified kind, contributing cards,
    /// and points awarded.
    pub fn new(kind: ScoreKind, cards: Vec<Card>, points: Points) -> Self {
        Self {
            kind,
            cards,
            points,
        }
    }

    /// Returns the kind of score this item represents.
    pub fn kind(&self) -> ScoreKind {
        self.kind
    }

    /// Returns an immutable reference to the cards that contributed to this scoring event.
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Returns the number of points awarded for this scoring item.
    pub fn points(&self) -> Points {
        self.points
    }
}
