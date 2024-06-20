use super::scorer::Scorer;
use super::scorer::constants::*;

use crate::domain::prelude::Cut;
use crate::types::prelude::{Face, HasFace, Points};

pub struct CutScorer(Cut);

impl CutScorer {
    pub fn new(cut: Cut) -> Self {
        Self(cut)
    }
}

impl Scorer for CutScorer {
    fn score(&self) -> Points {
        let cut = self.0;
        if cut.face() == Face::Jack {
            SCORE_HIS_HEELS_ON_CUT.into()
        } else {
            Points::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn his_heels_on_cut_for_non_jack() {
        let cards = vec![
            "AH", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H", "TH", "QH", "KH",
            "AC", "2C", "3C", "4C", "5C", "6C", "7C", "8C", "9C", "TC", "QC", "KC",
        ];
        cards
            .into_iter()
            .for_each(|c| {
                assert_eq!(CutScorer(Cut::from(c)).score(), 0.into())
            });
    }

    #[test]
    fn his_heels_on_cut_for_jack() {
        let cards = vec!["JH", "JC", "JD", "JS"];
        cards
            .into_iter()
            .for_each(|c| {
                assert_eq!(CutScorer(Cut::from(c)).score(), 2.into())
            });
    }
}
