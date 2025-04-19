use super::{constants::*, scorer::Scorer};
use crate::domain::{Cut, Face, ScoreReasons};

pub struct CutScorer(Cut);

impl CutScorer {
    pub const fn new(cut: Cut) -> Self {
        Self(cut)
    }
}

impl Scorer for CutScorer {
    fn score(&self) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let cut = self.0;
        if cut.face() == Face::Jack {
            reasons.with_his_heels(&[cut], SCORE_HIS_HEELS_ON_CUT.into());
        }

        reasons
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_not_score_his_heels_on_non_jack_cut() {
        let cards = vec![
            "AH", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H", "TH", "QH", "KH", "AC", "2C",
            "3C", "4C", "5C", "6C", "7C", "8C", "9C", "TC", "QC", "KC",
        ];
        cards.into_iter().for_each(|c| {
            assert_eq!(
                CutScorer(Cut::try_from(c).expect("valid cut"))
                    .score()
                    .points(),
                0.into()
            )
        });
    }

    #[test]
    fn should_score_his_heels_on_jack_cut() {
        let cards = vec!["JH", "JC", "JD", "JS"];
        cards.into_iter().for_each(|c| {
            assert_eq!(
                CutScorer(Cut::try_from(c).expect("valid cut"))
                    .score()
                    .points(),
                2.into()
            )
        });
    }
}
