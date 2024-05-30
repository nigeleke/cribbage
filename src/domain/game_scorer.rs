use super::card::{Face, Value};
use super::constants::PLAY_TARGET;
use super::game::Game;

pub(crate) struct GameScorer;

impl GameScorer {

    const SCORE_ZERO: usize = 0;
    const SCORE_HIS_HEELS_ON_CUT: usize = 2;
    const SCORE_FIFTEEN: usize = 2;
    const SCORE_THIRTY_ONE: usize = 2;
    const SCORE_UNDER_THIRTY_ONE: usize = 1;
  
    pub(crate) fn his_heels_on_cut_pre_play(game: &Game) -> usize {
        match game {
            Game::Playing(_, _, _, _, cut, _) => {
                if cut.face() == Face::Jack {
                    Self::SCORE_HIS_HEELS_ON_CUT
                } else {
                    Self::SCORE_ZERO           
                }
            },
            _ => unreachable!(),
        }
    }

    pub(crate) fn current_play(game: &Game) -> usize {
        Self::current_play_fifteen(game) +
            Self::end_of_play(game)
    }

    fn current_play_fifteen(game: &Game) -> usize {
        match game {
            Game::Playing(_, _, _, play_state, _, _) => {
                if play_state.running_total() == Value::from(15) {
                    Self::SCORE_FIFTEEN
                } else {
                    Self::SCORE_ZERO
                }
            },
            _ => unreachable!(),
        }
    }

    fn end_of_play(game: &Game) -> usize {
        match game {
            Game::Playing(_, _, hands, play_state, _, _) => {
                let total = play_state.running_total();
                let valid_plays = hands
                    .values()
                    .map(|cs| cs.cards()
                        .into_iter()
                        .filter(|c| c.value() + total <= Value::from(PLAY_TARGET))
                        .collect::<Vec<_>>())
                    .map(|h| h.len())
                    .sum::<usize>() > 0;
        
                if valid_plays {
                    GameScorer::SCORE_ZERO
                } else {
                    if total == Value::from(PLAY_TARGET) { GameScorer::SCORE_THIRTY_ONE }
                    else { GameScorer::SCORE_UNDER_THIRTY_ONE }
                }
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::builder::Builder;

    #[test]
    fn his_heels_on_cut_pre_play_scores_zero_for_non_jack() {
        let cards = Builder::card_chunks("AH2H3H4H5H6H7H8H9HTHQHKHAC2C3C4C5C6C7C8C9CTCQCKC");
        cards
            .iter()
            .for_each(|c| {
                let game = Builder::default()
                    .with_scores(0, 0)
                    .with_hands("", "")
                    .with_cut(&c)
                    .as_playing(1);
                assert_eq!(GameScorer::his_heels_on_cut_pre_play(&game), 0)
            });
    }

    #[test]
    fn his_heels_on_cut_pre_play_scores_for_jack() {
        let cards = Builder::card_chunks("JHJCJDJS");
        cards
            .iter()
            .for_each(|c| {
                let game = Builder::default()
                    .with_scores(0, 0)
                    .with_hands("", "")
                    .with_cut(&c)
                    .as_playing(1);
                assert_eq!(GameScorer::his_heels_on_cut_pre_play(&game), 2)
            });
    }

    #[test]
    fn play_fifteen_scores() {
        let game = Builder::default()
            .with_scores(0, 0)
            .with_hands("", "AS")
            .with_current_plays(&vec![(0, "JD"), (0, "5H")])
            .with_cut("AH")
            .as_playing(1);
        println!("play_fifteen_scores {}", game);
        assert_eq!(GameScorer::current_play(&game), 2)
    }

}