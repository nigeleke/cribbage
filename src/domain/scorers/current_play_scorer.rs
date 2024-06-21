use super::scorer::Scorer;
use super::constants::*;

use crate::domain::PlayState;
use crate::types::{Points, Rank};

pub struct CurrentPlayScorer(PlayState);

impl CurrentPlayScorer {
    pub fn new(play_state: &PlayState) -> Self {
        Self(play_state.clone())
    } 

    fn fifteens(&self) -> Points {
        let play_state = self.0.clone();
        if play_state.running_total() == 15.into() {
            SCORE_FIFTEEN.into()
        } else {
            Points::default()
        }
    }

    fn pairs(&self) -> Points {
        let play_state = self.0.clone();
        let current_plays = &play_state.current_plays();
        let mut current_plays = current_plays.iter().rev();
        let play = current_plays.next().unwrap();
        let previous = current_plays.take_while(|p| p.value() == play.value());
        match previous.count() {
            0 => Points::default(),
            1 => SCORE_PAIR.into(),
            2 => SCORE_ROYAL_PAIR.into(),
            3 => SCORE_DOUBLE_ROYAL_PAIR.into(),
            _ => unreachable!(),
        }
    }

    fn runs(&self) -> Points {
        let play_state = self.0.clone();

        let mut total = Points::default();

        let current_plays = &play_state.current_plays();

        for len in (MINIMUM_RUN_LENGTH..=current_plays.len()).rev() {
            let current_plays = current_plays.iter().rev();
            let mut plays = current_plays
                .map(|p| p.rank())
                .take(len)
                .collect::<Vec<_>>();
            plays.sort_by(|&a, &b| a.cmp(&b));

            let differences = plays
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();

            let sequential = differences.iter().all(|d| *d == Rank::new(1));
            if sequential {
                total += len.into();
                break;
            }
        }

        total
    }

}


impl Scorer for CurrentPlayScorer {
    fn score(&self) -> Points {
        self.fifteens() +
            self.pairs() +
            self.runs()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::{Card, Game};
    use crate::test::Builder;

    #[test]
    fn fifteens_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "JD"), (0, "5H")])
            .with_cut("AH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score(), 2.into())
    }

    #[test]
    fn pairs_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "JD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score(), 2.into())
    }

    #[test]
    fn royal_pair_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score(), 6.into())
    }

    #[test]
    fn double_royal_pair_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score(), 12.into())
    }

    #[test]
    fn run_n_scores() {
        let current_plays = vec![(0, "2C"), (0, "3C"), (0, "4C"), (0, "5C"), (0, "6C"), (0, "7C")];
        for len in 1..=current_plays.len() {
            let current_plays = current_plays.clone();
            let current_plays = current_plays.into_iter().take(len);
            let current_plays = Vec::from_iter(current_plays);
            let game = Builder::new(2)
                .with_scores(0, 0)
                .with_hands("KS", "KD")
                .with_current_plays(&current_plays)
                .with_cut("KH")
                .as_playing(1);
            let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
            assert_eq!(CurrentPlayScorer(play_state).score(), (if len < 3 { 0 } else { len }).into())
        }
    }

    #[test]
    fn rules_example_flush() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH", "KD")
            .with_cut("2H")
            .with_current_plays(&vec![(1, "TH"), (0, "9H"), (1, "QH")])
            .as_playing(0);
        let Game::Playing(_, dealer, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(dealer, Card::from("AH")).ok().unwrap();
        let Game::Playing(_, _, _, play_state, _, _) = game1.clone() else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score(), 0.into());
    }

}