use super::scorer::constants::*;

use crate::domain::prelude::{Card, Cards};
use crate::types::prelude::{Face, HasFace, HasRank, HasSuit, HasValue, Points, Value};

use itertools::*;

pub(super) struct CardsScorer;

impl CardsScorer {

    pub fn fifteens<T>(cards: &Cards<T>) -> Points
    where T: Clone {
        let mut total = Points::default();

        for n in 2..=cards.len() {
            for combination in cards.cards().into_iter().combinations(n) {
                let combination_total: Value = combination
                    .into_iter()
                    .map(|c| c.value())
                    .sum();

                if combination_total == 15.into() {
                    total += SCORE_FIFTEEN.into();
                }
            }
        }

        total
    }

    pub fn pairs<T>(cards: &Cards<T>) -> Points
    where T: Clone {
        let mut total = Points::default();

        for combination in cards.cards().into_iter().combinations(2) {
            let mut combination = combination.into_iter();
            let (one, two) = (combination.next().unwrap(), combination.next().unwrap());
            if one.face() == two.face() {
                total += SCORE_PAIR.into();
            }
        }

        total
    }

    pub fn runs<T>(cards: &Cards<T>) -> Points
    where T: Clone {
        let mut total = Points::default();

        let mut ranks = cards.cards()
            .iter()
            .map(|card| card.rank())
            .collect::<Vec<_>>();
        ranks.sort();

        for len in (MINIMUM_RUN_LENGTH..=ranks.len()).rev() {
            for combination in ranks.iter().combinations(len) {
                let differences = combination
                    .windows(2)
                    .map(|w| *w[1] - *w[0])
                    .collect::<Vec<_>>();

                let sequential = differences.iter().all(|d| *d == 1.into());
                if sequential {
                    total += len.into();
                }
            }

            if total != 0.into() { break; }
        }    

        total
    }

    pub fn flush<T>(cards: &Cards<T>) -> Points
    where T: Clone {
        let suit = cards.cards().first().map(|c| c.suit()).unwrap();
        let same_suit = cards.cards().iter().all(|c| c.suit() == suit);
        if same_suit {
            cards.len().into()
        } else {
            Points::default()
        }
    }

    pub fn his_heels<T>(cards: &Cards<T>, cut: Card) -> Points
    where T: Clone {
        let cards = cards.cards();
        let jacks = cards.iter().filter(|c| c.face() == Face::Jack);
        let suits = jacks.filter(|c| c.suit() == cut.suit());
        if suits.count() == 1 {
            SCORE_HIS_HEELS.into()
        } else {
            Points::default()
        }
    }
}
