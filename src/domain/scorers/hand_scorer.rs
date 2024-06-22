use super::cards_scorer::CardsScorer;
use super::scorer::Scorer;

use crate::domain::{Cut, Hand, ScoreReasons};
use crate::types::HasPoints;

pub struct HandScorer(Hand, Cut);

impl HandScorer {
    pub fn new(hand: &Hand, cut: Cut) -> Self {
        Self(hand.clone(), cut)
    }
}

impl Scorer for HandScorer {
    fn score(&self) -> ScoreReasons {
        let hand = self.0.clone();
        let cut = self.1;
        let mut all_cards = hand.clone();
        all_cards.add(&[cut]);

        let flush_hand = CardsScorer::flush(&hand);
        let flush_all = CardsScorer::flush(&all_cards);
        let flush_score = if flush_all.points() > flush_hand.points() { flush_all } else { flush_hand };

        CardsScorer::fifteens(&all_cards) +
            CardsScorer::pairs(&all_cards) +
            CardsScorer::runs(&all_cards) +
            flush_score + 
            CardsScorer::his_heels(&hand, cut)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_score_fifteens() {
        assert_eq!(HandScorer(Hand::from("7H8CAC2C"), Cut::from("4H")).score().points(), 4.into());
        assert_eq!(HandScorer(Hand::from("THJCKS5H"), Cut::from("4H")).score().points(), 6.into());
    }

    #[test]
    fn should_score_pairs() {
        assert_eq!(HandScorer(Hand::from("2H4C5C2C"), Cut::from("AH")).score().points(), 2.into());
        assert_eq!(HandScorer(Hand::from("TCASADTH"), Cut::from("AH")).score().points(), 8.into());
    }

    #[test]
    fn should_score_royal_pairs() {
        assert_eq!(HandScorer(Hand::from("2H2D5C2C"), Cut::from("AH")).score().points(), 6.into());
        assert_eq!(HandScorer(Hand::from("TCASADTH"), Cut::from("AH")).score().points(), 8.into());
    }

    #[test]
    fn should_score_double_royal_pairs() {
        assert_eq!(HandScorer(Hand::from("2H2C2D2S"), Cut::from("AH")).score().points(), 12.into());
        assert_eq!(HandScorer(Hand::from("TCASADTH"), Cut::from("AH")).score().points(), 8.into());
    }

    #[test]
    fn should_score_runs() {
        assert_eq!(HandScorer(Hand::from("JDQCKC2C"), Cut::from("AH")).score().points(), 3.into());
        assert_eq!(HandScorer(Hand::from("3C3S2D5H"), Cut::from("AH")).score().points(), 8.into());
    }

    #[test]
    fn should_score_flushes() {
        assert_eq!(HandScorer(Hand::from("2H4H6H8H"), Cut::from("TH")).score().points(), 5.into());
        assert_eq!(HandScorer(Hand::from("2D4D6D8D"), Cut::from("TH")).score().points(), 4.into());
    }

    #[test]
    fn should_score_his_heels() {
        assert_eq!(HandScorer(Hand::from("2D4H6HJH"), Cut::from("TH")).score().points(), 1.into());
        assert_eq!(HandScorer(Hand::from("2H4D6DJD"), Cut::from("TH")).score().points(), 0.into());
    }

    #[test]
    fn should_score_rules_example_eights_sevens_sixes() {
        assert_eq!(HandScorer(Hand::from("8H7C7D6S"), Cut::from("2H")).score().points(), 16.into());
    }

    #[test]
    fn should_score_rules_example_runs() {
        assert_eq!(HandScorer(Hand::from("JHQCKDAS"), Cut::from("2D")).score().points(), 3.into());
    }

    #[test]
    fn should_score_rules_example_flush() {
        assert_eq!(HandScorer(Hand::from("THQHKHAH"), Cut::from("2H")).score().points(), 5.into());
        assert_eq!(HandScorer(Hand::from("THQHKHAH"), Cut::from("2S")).score().points(), 4.into());
        assert_eq!(HandScorer(Hand::from("THQHKHAS"), Cut::from("2H")).score().points(), 0.into());
    }

    #[test]
    fn should_score_rules_example_perfect_29() {
        assert_eq!(HandScorer(Hand::from("5H5C5DJS"), Cut::from("5S")).score().points(), 29.into());
    }

}