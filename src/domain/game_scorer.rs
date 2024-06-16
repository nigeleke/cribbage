use super::card::{Card, Cut, Face, Rank, Value};
use super::cards::{Cards, Crib, Hand};
use super::plays::PlayState;

use crate::constants::*;

use itertools::*;

pub(crate) struct GameScorer;

impl GameScorer {
    const SCORE_ZERO: usize = 0;
    const SCORE_HIS_HEELS: usize = 1;
    const SCORE_HIS_HEELS_ON_CUT: usize = 2;
    const SCORE_FIFTEEN: usize = 2;
    const SCORE_THIRTY_ONE: usize = 2;
    const SCORE_UNDER_THIRTY_ONE: usize = 1;
    const SCORE_PAIR: usize = 2;
    const SCORE_TRIPLET: usize = 6;
    const SCORE_QUARTET: usize = 12;
    const MINIMUM_RUN_LENGTH: usize = 3;
  
    pub(crate) fn his_heels_on_cut_pre_play(cut: Cut) -> usize {
        if cut.face() == Face::Jack {
            Self::SCORE_HIS_HEELS_ON_CUT
        } else {
            Self::SCORE_ZERO           
        }
    }

    pub(crate) fn current_play(play_state: &PlayState) -> usize {
        Self::current_play_fifteen(play_state) +
            Self::current_play_pairs(play_state) +
            Self::current_play_runs(play_state)

    }

    fn current_play_fifteen(play_state: &PlayState) -> usize {
        if play_state.running_total() == Value::from(15) {
            Self::SCORE_FIFTEEN
        } else {
            Self::SCORE_ZERO
        }
    }

    fn current_play_pairs(play_state: &PlayState) -> usize {
        let current_plays = &play_state.current_plays();
        let mut current_plays = current_plays.iter().rev();
        let play = current_plays.next().unwrap();
        let previous = current_plays.take_while(|p| p.value() == play.value());
        match previous.count() {
            0 => Self::SCORE_ZERO,
            1 => Self::SCORE_PAIR,
            2 => Self::SCORE_TRIPLET,
            3 => Self::SCORE_QUARTET,
            _ => unreachable!(),
        }
    }

    fn current_play_runs(play_state: &PlayState) -> usize {
        let mut total = 0;

        let current_plays = &play_state.current_plays();

        for len in (Self::MINIMUM_RUN_LENGTH..=current_plays.len()).rev() {
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

            let sequential = differences.iter().all(|d| *d == Rank::from(1));
            if sequential {
                total += len;
                break;
            }
        }

        total
    }

    pub(crate) fn end_of_play(play_state: &PlayState) -> usize {
        if play_state.is_current_play_finished() {
            if play_state.running_total() == Value::from(PLAY_TARGET) {
                Self::SCORE_THIRTY_ONE
            } else {
                Self::SCORE_UNDER_THIRTY_ONE
            }
        } else {
            Self::SCORE_ZERO
        }
    }

    pub(crate) fn hand(cards: &Hand, cut: Cut) -> usize {
        let mut all_cards = cards.clone();
        all_cards.add(&[cut]);

        Self::cards_fifteen(&all_cards) +
            Self::cards_pairs(&all_cards) +
            Self::cards_runs(&all_cards) +
            std::cmp::max(
                Self::cards_flush(&all_cards),
                Self::cards_flush(cards)
            ) +
            Self::cards_his_heels(cards, cut)
    }

    pub(crate) fn crib(cards: &Crib, cut: Cut) -> usize {
        let mut all_cards = cards.clone();
        all_cards.add(&[cut]);

        Self::cards_fifteen(&all_cards) +
            Self::cards_pairs(&all_cards) +
            Self::cards_runs(&all_cards) +
            Self::cards_flush(&all_cards) +
            Self::cards_his_heels(cards, cut)
    }

    fn cards_fifteen<T>(cards: &Cards<T>) -> usize
    where T: Clone {
        let mut total = 0;

        for n in 2..=cards.len() {
            for combination in cards.cards().into_iter().combinations(n) {
                let combination_total: usize = combination
                    .into_iter()
                    .map(|c| Into::<usize>::into(c.value()))
                    .sum();

                if combination_total == 15 {
                    total += Self::SCORE_FIFTEEN;
                }
            }
        }

        total
    }

    fn cards_pairs<T>(cards: &Cards<T>) -> usize
    where T: Clone {
        let mut total = 0;

        for combination in cards.cards().into_iter().combinations(2) {
            let mut combination = combination.into_iter();
            let (one, two) = (combination.next().unwrap(), combination.next().unwrap());
            if one.face() == two.face() {
                total += Self::SCORE_PAIR;
            }
        }

        total
    }

    fn cards_runs<T>(cards: &Cards<T>) -> usize
    where T: Clone {
        let mut total = 0;

        let mut ranks = cards.cards()
            .iter()
            .map(|card| card.rank().into())
            .collect::<Vec<usize>>();
        ranks.sort();

        for len in (Self::MINIMUM_RUN_LENGTH..=ranks.len()).rev() {
            for combination in ranks.iter().combinations(len) {
                let differences = combination
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<_>>();

                let sequential = differences.iter().all(|d| *d == 1);
                if sequential {
                    total += len;
                }
            }

            if total != 0 { break; }
        }    

        total
    }

    fn cards_flush<T>(cards: &Cards<T>) -> usize
    where T: Clone {
        let suit = cards.cards().first().map(|c| c.suit()).unwrap();
        let same_suit = cards.cards().iter().all(|c| c.suit() == suit);
        if same_suit {
            cards.len()
        } else {
            Self::SCORE_ZERO
        }
    }

    fn cards_his_heels<T>(cards: &Cards<T>, cut: Card) -> usize
    where T: Clone {
        let cards = cards.cards();
        let jacks = cards.iter().filter(|c| c.face() == Face::Jack);
        let suits = jacks.filter(|c| c.suit() == cut.suit());
        if suits.count() == 1 {
            Self::SCORE_HIS_HEELS
        } else {
            Self::SCORE_ZERO
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::test::prelude::*;
    use crate::domain::game::Game;

    #[test]
    fn his_heels_on_cut_pre_play_scores_zero_for_non_jack() {
        let cards = vec![
            "AH", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H", "TH", "QH", "KH",
            "AC", "2C", "3C", "4C", "5C", "6C", "7C", "8C", "9C", "TC", "QC", "KC",
        ];
        cards
            .into_iter()
            .for_each(|c| {
                assert_eq!(GameScorer::his_heels_on_cut_pre_play(Card::from(c)), 0)
            });
    }

    #[test]
    fn his_heels_on_cut_pre_play_scores_for_jack() {
        let cards = vec!["JH", "JC", "JD", "JS"];
        cards
            .into_iter()
            .for_each(|c| {
                assert_eq!(GameScorer::his_heels_on_cut_pre_play(Card::from(c)), 2)
            });
    }

    #[test]
    fn current_play_fifteen_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "JD"), (0, "5H")])
            .with_cut("AH")
            .as_playing(None);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(GameScorer::current_play(&play_state), 2)
    }

    #[test]
    fn current_play_pairs_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "JD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(None);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(GameScorer::current_play(&play_state), 2)
    }

    #[test]
    fn current_play_triplets_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(None);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(GameScorer::current_play(&play_state), 6)
    }

    #[test]
    fn current_play_quartets_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(None);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(GameScorer::current_play(&play_state), 12)
    }

    #[test]
    fn current_play_run_n_scores() {
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
                .as_playing(None);
            let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
            assert_eq!(GameScorer::current_play(&play_state), if len < 3 { 0 } else { len })
        }
    }

    #[test]
    fn end_of_play_target_not_reached_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
            .with_cut("KH")
            .as_playing(None);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(GameScorer::end_of_play(&play_state), 1)
    }

    #[test]
    fn end_of_play_target_reached_scores() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("", "")
            .with_current_plays(&vec![(0, "KC"), (0, "KD"), (0, "KH"), (0, "AS")])
            .with_cut("KS")
            .as_playing(None);
        let Game::Playing(_, _, _, play_state, _, _) = game else { panic!("Unexpected state") };
        assert_eq!(GameScorer::end_of_play(&play_state), 2)
    }

    #[test]
    fn hand_fifteen_scores() {
        assert_eq!(GameScorer::hand(&Hand::from("7H8CAC2C"), Card::from("4H")), 4);
        assert_eq!(GameScorer::hand(&Hand::from("THJCKS5H"), Card::from("4H")), 6);
    }

    #[test]
    fn hand_pairs_scores() {
        assert_eq!(GameScorer::hand(&Hand::from("2H4C5C2C"), Card::from("AH")), 2);
        assert_eq!(GameScorer::hand(&Hand::from("TCASADTH"), Card::from("AH")), 8);
    }

    #[test]
    fn hand_triplets_scores() {
        assert_eq!(GameScorer::hand(&Hand::from("2H2D5C2C"), Card::from("AH")), 6);
        assert_eq!(GameScorer::hand(&Hand::from("TCASADTH"), Card::from("AH")), 8);
    }

    #[test]
    fn hand_quartets_scores() {
        assert_eq!(GameScorer::hand(&Hand::from("2H2C2D2S"), Card::from("AH")), 12);
        assert_eq!(GameScorer::hand(&Hand::from("TCASADTH"), Card::from("AH")), 8);
    }

    #[test]
    fn hand_runs_scores() {
        assert_eq!(GameScorer::hand(&Hand::from("JDQCKC2C"), Card::from("AH")), 3);
        assert_eq!(GameScorer::hand(&Hand::from("3C3S2D5H"), Card::from("AH")), 8);
    }

    #[test]
    fn hand_flush_scores() {
        assert_eq!(GameScorer::hand(&Hand::from("2H4H6H8H"), Card::from("TH")), 5);
        assert_eq!(GameScorer::hand(&Hand::from("2D4D6D8D"), Card::from("TH")), 4);
    }

    #[test]
    fn hand_his_heels_scores() {
        assert_eq!(GameScorer::hand(&Hand::from("2D4H6HJH"), Card::from("TH")), 1);
        assert_eq!(GameScorer::hand(&Hand::from("2H4D6DJD"), Card::from("TH")), 0);
    }

    #[test]
    fn crib_fifteen_scores() {
        assert_eq!(GameScorer::crib(&Crib::from("7H8CAC2C"), Card::from("4H")), 4);
        assert_eq!(GameScorer::crib(&Crib::from("THJCKS5H"), Card::from("4H")), 6);
    }

    #[test]
    fn crib_pairs_scores() {
        assert_eq!(GameScorer::crib(&Crib::from("2H4C5C2C"), Card::from("AH")), 2);
        assert_eq!(GameScorer::crib(&Crib::from("TCASADTH"), Card::from("AH")), 8);
    }

    #[test]
    fn crib_triplets_scores() {
        assert_eq!(GameScorer::crib(&Crib::from("2H2D5C2C"), Card::from("AH")), 6);
        assert_eq!(GameScorer::crib(&Crib::from("TCASADTH"), Card::from("AH")), 8);
    }

    #[test]
    fn crib_quartets_scores() {
        assert_eq!(GameScorer::crib(&Crib::from("2H2C2D2S"), Card::from("AH")), 12);
        assert_eq!(GameScorer::crib(&Crib::from("TCASADTH"), Card::from("AH")), 8);
    }

    #[test]
    fn crib_runs_scores() {
        assert_eq!(GameScorer::crib(&Crib::from("JDQCKC2C"), Card::from("AH")), 3);
        assert_eq!(GameScorer::crib(&Crib::from("3C3S2D5H"), Card::from("AH")), 8);
    }

    #[test]
    fn crib_flush_scores() {
        assert_eq!(GameScorer::crib(&Crib::from("2H4H6H8H"), Card::from("TH")), 5);
        assert_eq!(GameScorer::crib(&Crib::from("2D4D6D8D"), Card::from("TH")), 0);
    }

    #[test]
    fn crib_his_heels_scores() {
        assert_eq!(GameScorer::crib(&Crib::from("2D4H6HJH"), Card::from("TH")), 1);
        assert_eq!(GameScorer::crib(&Crib::from("2H4D6DJD"), Card::from("TH")), 0);
    }
 
    #[test]
    fn rules_example_eights_sevens_sixes() {
        assert_eq!(GameScorer::hand(&Hand::from("8H7C7D6S"), Card::from("2H")), 16);
    }

    #[test]
    fn rules_example_runs() {
        assert_eq!(GameScorer::hand(&Hand::from("JHQCKDAS"), Card::from("2D")), 3);
    }

    #[test]
    fn rules_example_flush() {
        assert_eq!(GameScorer::hand(&Hand::from("THQHKHAH"), Card::from("2H")), 5);
        assert_eq!(GameScorer::hand(&Hand::from("THQHKHAH"), Card::from("2S")), 4);
        assert_eq!(GameScorer::hand(&Hand::from("THQHKHAS"), Card::from("2H")), 0);
        
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH", "KD")
            .with_cut("2H")
            .with_current_plays(&vec![(1, "TH"), (0, "9H"), (1, "QH")])
            .as_playing(Some(0));
        let Game::Playing(_, dealer, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(dealer, Card::from("AH")).ok().unwrap();
        let Game::Playing(_, _, _, play_state, _, _) = game1.clone() else { panic!("Unexpected state") };
        assert_eq!(GameScorer::current_play(&play_state), 0);
    }

    #[test]
    fn rules_example_perfect_29() {
        assert_eq!(GameScorer::hand(&Hand::from("5H5C5DJS"), Card::from("5S")), 29);
    }
}