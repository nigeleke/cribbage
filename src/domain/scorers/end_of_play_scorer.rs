use super::scorer::Scorer;
use super::constants::*;

use crate::constants::PLAY_TARGET;
use crate::domain::{PlayState, ScoreReasons};

pub struct EndOfPlayScorer(PlayState);

impl EndOfPlayScorer {
    pub fn new(play_state: &PlayState) -> Self {
        Self(play_state.clone())
    }
}

impl Scorer for EndOfPlayScorer {
    fn score(&self) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let play_state = &self.0;

        if play_state.is_current_play_finished() {
            let cards = play_state.current_plays()
                .iter()
                .map(|p| p.card())
                .collect::<Vec<_>>();

            if play_state.running_total() == PLAY_TARGET.into() {
                reasons.with_end_of_play(&cards, SCORE_THIRTY_ONE.into());
            } else {
                reasons.with_end_of_play(&cards, SCORE_UNDER_THIRTY_ONE.into());
            }
        }

        reasons
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::Game;
    use crate::test::Builder;
    use crate::types::HasPoints;

    #[test]
    fn should_score_when_target_not_reached() {
        let game = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(EndOfPlayScorer(play_state).score().points(), 1.into())
    }

    #[test]
    fn should_score_when_target_reached() {
        let game = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "KC"), (0, "KD"), (0, "KH"), (0, "AS")])
            .with_cut("KS")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(EndOfPlayScorer(play_state).score().points(), 2.into())
    }

}