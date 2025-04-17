use super::cards_scorer::CardsScorer;
use super::scorer::Scorer;

use crate::domain::{Crib, Cut, ScoreReasons};

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

impl Scorer for CribScorer {
    fn score(&self) -> ScoreReasons {
        let crib = self.crib.clone();
        let cut = self.cut;
        let mut all_cards = crib.clone();
        all_cards.add(&[cut]);

        CardsScorer::fifteens(all_cards.as_ref())
            + CardsScorer::pairs(all_cards.as_ref())
            + CardsScorer::runs(all_cards.as_ref())
            + CardsScorer::flush(all_cards.as_ref())
            + CardsScorer::his_heels(crib.as_ref(), cut)
    }
}
