use super::constants::*;
use super::scorer::Scorer;

use crate::domain::{PlayState, Points, ScoreReasons};

pub struct CurrentPlayScorer(PlayState);

impl CurrentPlayScorer {
    pub fn new(play_state: &PlayState) -> Self {
        Self(play_state.clone())
    }

    fn fifteens(&self) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let play_state = &self.0;
        let cards = play_state
            .current_plays()
            .into_iter()
            .map(|p| p.card())
            .collect::<Vec<_>>();

        if play_state.running_total() == 15.into() {
            reasons.with_fifteen(&cards, SCORE_FIFTEEN.into())
        }

        reasons
    }

    fn pairs(&self) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let play_state = &self.0;
        let mut cards = play_state
            .current_plays()
            .into_iter()
            .map(|p| p.card())
            .rev();

        let first = cards.next().unwrap();
        let matching = cards.take_while(|c| c.face() == first.face());

        let mut cards = Vec::from_iter(matching);
        cards.push(first);

        let points = match cards.len() {
            1 => Points::default(),
            2 => SCORE_PAIR.into(),
            3 => SCORE_ROYAL_PAIR.into(),
            4 => SCORE_DOUBLE_ROYAL_PAIR.into(),
            _ => unreachable!(),
        };

        if points != 0.into() {
            reasons.with_pairs(&cards, points)
        }

        reasons
    }

    fn runs(&self) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let play_state = &self.0;

        let current_plays = &play_state.current_plays();

        for len in (MINIMUM_RUN_LENGTH..=current_plays.len()).rev() {
            let current_plays = current_plays.iter().rev();
            let mut cards = current_plays
                .map(|p| p.card())
                .take(len)
                .collect::<Vec<_>>();

            cards.sort_by(|&a, &b| a.rank().cmp(&b.rank()));

            let differences = cards
                .windows(2)
                .map(|w| *w[1].rank() - *w[0].rank())
                .collect::<Vec<_>>();

            let sequential = differences.iter().all(|d| *d == 1);
            if sequential {
                reasons.with_run(&cards, len.into());
                break;
            }
        }

        reasons
    }
}

impl Scorer for CurrentPlayScorer {
    fn score(&self) -> ScoreReasons {
        self.fifteens() + self.pairs() + self.runs()
    }
}
