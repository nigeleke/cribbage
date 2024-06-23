use super::scorer::Scorer;
use super::constants::*;

use crate::domain::{PlayState, ScoreReasons};
use crate::types::{Points, Rank, HasFace, HasRank};

pub struct CurrentPlayScorer(PlayState);

impl CurrentPlayScorer {
    pub fn new(play_state: &PlayState) -> Self {
        Self(play_state.clone())
    } 

    fn fifteens(&self) -> ScoreReasons {
        let mut reasons = ScoreReasons::default();

        let play_state = &self.0;
        let cards = play_state.current_plays()
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
        let mut cards = play_state.current_plays()
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
                .map(|w| w[1].rank() - w[0].rank())
                .collect::<Vec<_>>();

            let sequential = differences.iter().all(|d| *d == Rank::new(1));
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
    use crate::types::HasPoints;

    #[test]
    fn should_score_fifteens() {
        let game = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("", "")
            .with_current_plays(&[(0, "JD"), (0, "5H")])
            .with_cut("AH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score().points(), 2.into())
    }

    #[test]
    fn should_score_pairs() {
        let game = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("", "")
            .with_current_plays(&[(0, "JD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score().points(), 2.into())
    }

    #[test]
    fn should_score_royal_pairs() {
        let game = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("", "")
            .with_current_plays(&[(0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score().points(), 6.into())
    }

    #[test]
    fn should_score_double_royal_pairs() {
        let game = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("", "")
            .with_current_plays(&[(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score().points(), 12.into())
    }

    #[test]
    fn should_score_runs() {
        let current_plays = &[(0, "2C"), (0, "3C"), (0, "4C"), (0, "5C"), (0, "6C"), (0, "7C")];
        for len in 1..=current_plays.len() {
            let current_plays = current_plays.clone();
            let current_plays = current_plays.into_iter().take(len);
            let current_plays = Vec::from_iter(current_plays);
            let game = Builder::new(2)
                .with_peggings(0, 0)
                .with_hands("KS", "KD")
                .with_current_plays(&current_plays)
                .with_cut("KH")
                .as_playing(1);
            let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
            assert_eq!(CurrentPlayScorer(play_state).score().points(), (if len < 3 { 0 } else { len }).into())
        }
    }

    #[test]
    fn should_score_runs_unordered() {
        let game = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("KS", "KD")
            .with_current_plays(&[(0, "3S"), (0, "2C"), (0, "AS")])
            .with_cut("KH")
            .as_playing(1);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score().points(), 3.into())
    }

    #[test]
    fn should_score_rules_example_flush() {
        let game0 = Builder::new(2)
            .with_peggings(0, 0)
            .with_hands("AH", "KD")
            .with_cut("2H")
            .with_current_plays(&[(1, "TH"), (0, "9H"), (1, "QH")])
            .as_playing(0);
        let Game::Playing(_, dealer, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(dealer, Card::from("AH")).ok().unwrap();
        let Game::Playing(_, _, _, play_state, _, _) = game1.clone() else { panic!("Unexpected state") };
        assert_eq!(CurrentPlayScorer(play_state).score().points(), 0.into());
    }

}