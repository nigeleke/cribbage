use super::cards_scorer::CardsScorer;
use super::scorer::Scorer;

use crate::domain::{Crib, Cut};
use crate::types::Points;

pub struct CribScorer(Crib, Cut);

impl CribScorer {
    pub fn new(crib: &Crib, cut: Cut) -> Self {
        Self(crib.clone(), cut)
    }
}

impl Scorer for CribScorer {
    fn score(&self) -> Points {
        let crib = self.0.clone();
        let cut = self.1;
        let mut all_cards = crib.clone();
        all_cards.add(&[cut]);

        CardsScorer::fifteens(&all_cards) +
            CardsScorer::pairs(&all_cards) +
            CardsScorer::runs(&all_cards) +
            CardsScorer::flush(&all_cards) +
            CardsScorer::his_heels(&crib, cut)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_score_fifteens() {
        assert_eq!(CribScorer(Crib::from("7H8CAC2C"), Cut::from("4H")).score(), 4.into());
        assert_eq!(CribScorer(Crib::from("THJCKS5H"), Cut::from("4H")).score(), 6.into());
    }

    #[test]
    fn should_score_pairs() {
        assert_eq!(CribScorer(Crib::from("2H4C5C2C"), Cut::from("AH")).score(), 2.into());
        assert_eq!(CribScorer(Crib::from("TCASADTH"), Cut::from("AH")).score(), 8.into());
    }

    #[test]
    fn should_score_royal_pairs() {
        assert_eq!(CribScorer(Crib::from("2H2D5C2C"), Cut::from("AH")).score(), 6.into());
        assert_eq!(CribScorer(Crib::from("TCASADTH"), Cut::from("AH")).score(), 8.into());
    }

    #[test]
    fn should_score_double_royal_pairs() {
        assert_eq!(CribScorer(Crib::from("2H2C2D2S"), Cut::from("AH")).score(), 12.into());
        assert_eq!(CribScorer(Crib::from("TCASADTH"), Cut::from("AH")).score(), 8.into());
    }

    #[test]
    fn should_score_runs() {
        assert_eq!(CribScorer(Crib::from("JDQCKC2C"), Cut::from("AH")).score(), 3.into());
        assert_eq!(CribScorer(Crib::from("3C3S2D5H"), Cut::from("AH")).score(), 8.into());
    }

    #[test]
    fn should_score_flushes() {
        assert_eq!(CribScorer(Crib::from("2H4H6H8H"), Cut::from("TH")).score(), 5.into());
        assert_eq!(CribScorer(Crib::from("2D4D6D8D"), Cut::from("TH")).score(), 0.into());
    }

    #[test]
    fn should_score_his_heels() {
        assert_eq!(CribScorer(Crib::from("2D4H6HJH"), Cut::from("TH")).score(), 1.into());
        assert_eq!(CribScorer(Crib::from("2H4D6DJD"), Cut::from("TH")).score(), 0.into());
    }

}