use super::card::{Cut, Face, Value};
use super::constants::PLAY_TARGET;
use super::plays::PlayState;

pub(crate) struct GameScorer;

impl GameScorer {

    const SCORE_ZERO: usize = 0;
    const SCORE_HIS_HEELS_ON_CUT: usize = 2;
    const SCORE_FIFTEEN: usize = 2;
    const SCORE_THIRTY_ONE: usize = 2;
    const SCORE_UNDER_THIRTY_ONE: usize = 1;
  
    pub(crate) fn his_heels_on_cut_pre_play(cut: Cut) -> usize {
        if cut.face() == Face::Jack {
            Self::SCORE_HIS_HEELS_ON_CUT
        } else {
            Self::SCORE_ZERO           
        }
    }

    pub(crate) fn current_play(play_state: &PlayState) -> usize {
        Self::current_play_fifteen(play_state)
    }

    fn current_play_fifteen(play_state: &PlayState) -> usize {
        if play_state.running_total() == Value::from(15) {
            Self::SCORE_FIFTEEN
        } else {
            Self::SCORE_ZERO
        }
    }

    pub(crate) fn end_of_play(play_state: &PlayState) -> usize {
        let total = play_state.running_total();

        if total == Value::from(PLAY_TARGET) {
            Self::SCORE_THIRTY_ONE
        } else {
            Self::SCORE_UNDER_THIRTY_ONE
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::domain::builder::Builder;
    use crate::domain::game::Game;

    #[test]
    fn his_heels_on_cut_pre_play_scores_zero_for_non_jack() {
        let cards = Builder::card_chunks("AH2H3H4H5H6H7H8H9HTHQHKHAC2C3C4C5C6C7C8C9CTCQCKC");
        cards
            .iter()
            .for_each(|c| {
                assert_eq!(GameScorer::his_heels_on_cut_pre_play(Builder::card(c)), 0)
            });
    }

    #[test]
    fn his_heels_on_cut_pre_play_scores_for_jack() {
        let cards = Builder::card_chunks("JHJCJDJS");
        cards
            .iter()
            .for_each(|c| {
                assert_eq!(GameScorer::his_heels_on_cut_pre_play(Builder::card(c)), 2)
            });
    }

    #[test]
    fn play_fifteen_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "AS")
            .with_current_plays(&vec![(0, "JD"), (0, "5H")])
            .with_cut("AH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(GameScorer::current_play(&play_state), 2)
    }

}