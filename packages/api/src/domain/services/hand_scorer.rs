use super::{cards_scorer::CardsScorer, common::ScoringRule};
use crate::domain::{
    HasPoints, ScoreComposition,
    game::cards::{Cut, Hand},
};

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

impl ScoringRule for HandScorer {
    fn score(&self) -> ScoreComposition {
        let hand = Vec::from_iter(self.hand.into_iter().copied());
        let cut = self.cut;
        let mut all_cards = hand.clone();
        all_cards.push(cut);

        let flush_hand = CardsScorer::flush(&hand);
        let flush_all = CardsScorer::flush(&all_cards);
        let flush_score = if flush_all.points() > flush_hand.points() {
            flush_all
        } else {
            flush_hand
        };

        CardsScorer::fifteens(&all_cards)
            + CardsScorer::pairs(&all_cards)
            + CardsScorer::runs(&all_cards)
            + flush_score
            + CardsScorer::his_heels(&hand, cut)
    }
}
