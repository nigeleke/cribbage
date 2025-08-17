use super::constants::*;
use crate::domain::game::{
    cards::{Card, Hand, HasFace, HasRank, HasSuit, HasValue, Value},
    scoreboard::Points,
    scoring::ScoreComposition,
};
use itertools::*;

pub(super) struct CardsScorer;

impl CardsScorer {
    pub fn fifteens(cards: &[Card]) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        for n in 2..=cards.len() {
            for combination in cards.iter().combinations(n) {
                let combination_total = combination.iter().map(|c| c.value()).sum::<Value>();

                if combination_total == Value::from(15) {
                    let cards = combination.into_iter().copied().collect::<Vec<_>>();
                    reasons.with_fifteen(&cards, Points::from(SCORE_FIFTEEN));
                }
            }
        }

        reasons
    }

    pub fn pairs(cards: &[Card]) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        for combination in cards.iter().combinations(2) {
            let mut combination = combination.into_iter();
            let (one, two) = (
                combination.next().expect("combination next"),
                combination.next().expect("combination next"),
            );
            if one.face() == two.face() {
                let cards = [*one, *two];
                reasons.with_pairs(&cards, SCORE_PAIR.into());
            }
        }

        reasons
    }

    pub fn runs(cards: &[Card]) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        let mut cards = Hand::from(cards);
        cards.sort_by_rank();

        for len in (MINIMUM_RUN_LENGTH..=cards.len()).rev() {
            for combination in cards.into_iter().combinations(len) {
                let sequential = combination
                    .windows(2)
                    .map(|w| w[1].rank() - w[0].rank())
                    .all(|d| d == 1);

                if sequential {
                    let points = Points::from(combination.len());
                    let cards = combination.into_iter().copied().collect::<Vec<_>>();
                    reasons.with_run(&cards, points);
                }
            }

            if !reasons.is_empty() {
                break;
            }
        }

        reasons
    }

    pub fn flush(cards: &[Card]) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        let mut suits = cards.iter().map(Card::suit);
        if suits.all_equal() {
            reasons.with_flush(cards, cards.len().into())
        }

        reasons
    }

    pub fn his_heels(cards: &[Card], cut: Card) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        let jacks = cards.iter().filter(|c| c.face().is_jack());
        let suits = jacks.filter(|c| c.suit() == cut.suit());
        if suits.count() == 1 {
            reasons.with_his_heels(cards, SCORE_HIS_HEELS.into())
        }

        reasons
    }
}
