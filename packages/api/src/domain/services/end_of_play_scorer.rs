use super::{common::ScoringRule, constants::*};
use crate::{
    constants::*,
    domain::game::{cards::Value, plays::PlayState, scoreboard::Points, scoring::ScoreComposition},
};

pub struct EndOfPlayScorer(PlayState);

impl EndOfPlayScorer {
    pub fn new(play_state: &PlayState) -> Self {
        Self(play_state.clone())
    }
}

impl ScoringRule for EndOfPlayScorer {
    fn score(&self) -> ScoreComposition {
        let mut reasons = ScoreComposition::default();

        let play_state = &self.0;

        if play_state.is_current_play_finished() {
            let cards = play_state
                .current_plays()
                .iter()
                .map(|p| p.card())
                .collect::<Vec<_>>();

            if play_state.running_total() == Value::from(PLAY_TARGET) {
                reasons.with_end_of_play(&cards, Points::from(SCORE_THIRTY_ONE));
            } else {
                reasons.with_end_of_play(&cards, Points::from(SCORE_UNDER_THIRTY_ONE));
            }
        }

        reasons
    }
}
