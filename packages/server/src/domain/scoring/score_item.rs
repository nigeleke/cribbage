use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{Card, Points, ScoreKind},
};

/// Represents a single scoring event in the game.
///
/// Each `ScoreItem` captures:
/// - the type of score (`kind`),
/// - the set of cards that contributed to the score (`cards`),
/// - and the number of points awarded (`points`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    #[must_use]
    pub fn kind(&self) -> ScoreKind {
        self.kind
    }

    /// Returns an immutable reference to the cards that contributed to this scoring event.
    #[must_use]
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Returns the number of points awarded for this scoring item.
    #[must_use]
    pub fn points(&self) -> Points {
        self.points
    }
}

impl std::fmt::Display for ScoreItem {
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
