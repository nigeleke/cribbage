use super::{common::ScoringRule, constants::*};
use crate::domain::game::{
    cards::{HasFace, HasRank, Value},
    plays::PlayState,
    scoreboard::Points,
    scoring::ScoreComposition,
};

pub struct CurrentPlayScorer(PlayState);

impl CurrentPlayScorer {
    pub fn new(play_state: &PlayState) -> Self {
        Self(play_state.clone())
    }

    fn fifteens(&self) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        let play_state = &self.0;
        let cards = play_state
            .current_plays()
            .into_iter()
            .map(|p| p.card())
            .collect::<Vec<_>>();

        if play_state.running_total() == Value::from(15) {
            reasons.with_fifteen(&cards, Points::from(SCORE_FIFTEEN))
        }

        reasons
    }

    fn pairs(&self) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        let play_state = &self.0;
        let mut cards = play_state
            .current_plays()
            .into_iter()
            .map(|p| p.card())
            .rev();

        let first = cards.next().expect("cards.next");
        let matching = cards.take_while(|c| c.face() == first.face());

        let mut cards = Vec::from_iter(matching);
        cards.push(first);

        let points = match cards.len() {
            1 => Points::default(),
            2 => SCORE_PAIR.into(),
            3 => SCORE_ROYAL_PAIR.into(),
            4 => SCORE_DOUBLE_ROYAL_PAIR.into(),
            _ => unreachable!("never >4 cards with same face"),
        };

        if points != 0.into() {
            reasons.with_pairs(&cards, points)
        }

        reasons
    }

    fn runs(&self) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

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
                .map(|w| w[1].rank() - w[0].rank())
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

impl ScoringRule for CurrentPlayScorer {
    fn score(&self) -> ScoreComposition {
        self.fifteens() + self.pairs() + self.runs()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        card,
        domain::{
            PLAYER0, PLAYER1,
            game::cards::{Card, Hand},
        },
        hand,
    };
    use std::str::FromStr;

    #[test]
    #[should_panic]
    fn impossible_pairs_will_panic() {
        let hand1 = hand!("AHACADASAH");
        let hand2 = hand!("");

        let mut play_state = PlayState::new(PLAYER0)
            .with_pending_plays(PLAYER0, &hand1)
            .with_pending_plays(PLAYER1, &hand2);
        play_state.play(card!("AH"));
        play_state.play(card!("AC"));
        play_state.play(card!("AD"));
        play_state.play(card!("AS"));
        play_state.play(card!("AH"));

        let _ = CurrentPlayScorer::new(&play_state).score();
    }
}
