use super::cards_scorer::CardsScorer;
use super::scorer::Scorer;

use crate::domain::{Cut, Hand, ScoreReasons};

pub struct HandScorer {
    hand: Hand,
    cut: Cut,
}

impl HandScorer {
    pub fn new(hand: &Hand, cut: Cut) -> Self {
        Self {
            hand: hand.clone(),
            cut,
        }
    }
}

impl Scorer for HandScorer {
    fn score(&self) -> ScoreReasons {
        let hand = self.hand.clone();
        let cut = self.cut;
        let mut all_cards = hand.clone();
        all_cards.add(&[cut]);

        let flush_hand = CardsScorer::flush(hand.as_ref());
        let flush_all = CardsScorer::flush(all_cards.as_ref());
        let flush_score = if flush_all.points() > flush_hand.points() {
            flush_all
        } else {
            flush_hand
        };

        CardsScorer::fifteens(all_cards.as_ref())
            + CardsScorer::pairs(all_cards.as_ref())
            + CardsScorer::runs(all_cards.as_ref())
            + flush_score
            + CardsScorer::his_heels(hand.as_ref(), cut)
    }
}
