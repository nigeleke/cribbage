use super::{common::ScoringRule, constants::*};
use crate::domain::game::{
    cards::{Cut, HasFace},
    scoreboard::Points,
    scoring::ScoreComposition,
};

pub struct CutScorer(Cut);

impl CutScorer {
    pub const fn new(cut: Cut) -> Self {
        Self(cut)
    }
}

impl ScoringRule for CutScorer {
    fn score(&self) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        let cut = self.0;
        if cut.face().is_jack() {
            reasons.with_his_heels(&[cut], Points::from(SCORE_HIS_HEELS_ON_CUT));
        }

        reasons
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        card,
        domain::{HasPoints, game::cards::Card},
    };
    use std::str::FromStr;

    #[test]
    fn should_not_score_his_heels_on_non_jack_cut() {
        let cards = vec![
            "AH", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H", "TH", "QH", "KH", "AC", "2C",
            "3C", "4C", "5C", "6C", "7C", "8C", "9C", "TC", "QC", "KC",
        ];
        cards
            .into_iter()
            .for_each(|c| assert_eq!(CutScorer(card!(c)).score().points(), Points::from(0)));
    }

    #[test]
    fn should_score_his_heels_on_jack_cut() {
        let cards = vec!["JH", "JC", "JD", "JS"];
        cards
            .into_iter()
            .for_each(|c| assert_eq!(CutScorer(card!(c)).score().points(), 2.into()));
    }
}
