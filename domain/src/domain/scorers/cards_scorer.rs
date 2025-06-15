use super::constants::*;
use crate::domain::{Card, Face, ScoreReasons, Value};
use itertools::*;

pub(super) struct CardsScorer;

impl CardsScorer {
    pub fn fifteens(cards: &[Card]) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        for n in 2..=cards.len() {
            for combination in cards.as_ref().iter().combinations(n) {
                let combination_total: Value = combination.iter().map(|c| c.value()).sum();

                if combination_total == 15.into() {
                    let cards = combination.iter().map(|c| **c).collect::<Vec<_>>();
                    reasons.with_fifteen(&cards, SCORE_FIFTEEN.into());
                }
            }
        }

        reasons
    }

    pub fn pairs(cards: &[Card]) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        for combination in cards.as_ref().iter().combinations(2) {
            let mut combination = combination.into_iter();
            let (one, two) = (
                combination.next().expect("combination next"),
                combination.next().expect("combination next"),
            );
            if one.face() == two.face() {
                reasons.with_pairs(&[*one, *two], SCORE_PAIR.into());
            }
        }

        reasons
    }

    pub fn runs(cards: &[Card]) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let mut ranks = cards
            .as_ref()
            .iter()
            .map(|card| card.rank())
            .collect::<Vec<_>>();
        ranks.sort();

        for len in (MINIMUM_RUN_LENGTH..=ranks.len()).rev() {
            for combination in ranks.iter().combinations(len) {
                let differences = combination
                    .windows(2)
                    .map(|w| w[1].value() - w[0].value())
                    .collect::<Vec<_>>();

                let sequential = differences.iter().all(|d| *d == 1);
                if sequential {
                    // TODO: Record cards....
                    reasons.with_run(&[], combination.len().into());
                }
            }

            if !reasons.is_empty() {
                break;
            }
        }

        reasons
    }

    pub fn flush(cards: &[Card]) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let suit = cards.first().map(|c| c.suit()).expect("cards.first");
        let same_suit = cards.iter().all(|c| c.suit() == suit);
        if same_suit {
            reasons.with_flush(cards, cards.len().into())
        }

        reasons
    }

    pub fn his_heels(cards: &[Card], cut: Card) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let jacks = cards.iter().filter(|c| c.face() == Face::Jack);
        let suits = jacks.filter(|c| c.suit() == cut.suit());
        if suits.count() == 1 {
            reasons.with_his_heels(cards, SCORE_HIS_HEELS.into())
        }

        reasons
    }
}
