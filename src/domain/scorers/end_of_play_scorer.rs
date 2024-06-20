use super::scorer::Scorer;
use super::scorer::constants::*;

use crate::constants::PLAY_TARGET;
use crate::domain::prelude::PlayState;
use crate::types::prelude::Points;

pub struct EndOfPlayScorer(PlayState);

impl EndOfPlayScorer {
    pub fn new(play_state: &PlayState) -> Self {
        Self(play_state.clone())
    }
}

impl Scorer for EndOfPlayScorer {
    fn score(&self) -> Points {
        let play_state = self.0.clone();
        if play_state.is_current_play_finished() {
            if play_state.running_total() == PLAY_TARGET.into() {
                SCORE_THIRTY_ONE.into()
            } else {
                SCORE_UNDER_THIRTY_ONE.into()
            }
        } else {
            Points::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::prelude::Game;
    use crate::test::prelude::Builder;

    #[test]
    fn target_not_reached() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(EndOfPlayScorer(play_state).score(), 1.into())
    }

    #[test]
    fn target_reached() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "KC"), (0, "KD"), (0, "KH"), (0, "AS")])
            .with_cut("KS")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(EndOfPlayScorer(play_state).score(), 2.into())
    }

}