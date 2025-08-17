use super::{cards_scorer::CardsScorer, common::ScoringRule};
use crate::domain::game::{
    cards::{Crib, Cut},
    scoring::ScoreComposition,
};

pub struct CribScorer {
    crib: Crib,
    cut: Cut,
}

impl CribScorer {
    pub fn new(crib: &Crib, cut: Cut) -> Self {
        Self {
            crib: crib.clone(),
            cut,
        }
    }
}

impl ScoringRule for CribScorer {
    fn score(&self) -> ScoreComposition {
        let crib = Vec::from_iter(self.crib.into_iter().copied());
        let cut = self.cut;
        let mut all_cards = crib.clone();
        all_cards.push(cut);

        CardsScorer::fifteens(&all_cards)
            + CardsScorer::pairs(&all_cards)
            + CardsScorer::runs(&all_cards)
            + CardsScorer::flush(&all_cards)
            + CardsScorer::his_heels(&crib, cut)
    }
}
