use super::constants::*;
use super::card::{Card, Cuts, Rank};
use super::cards::{Crib, Deck, Hand, Hands};
use super::format::format_hashmap;
use super::player::Player;
use super::plays::PlayState;
use super::result::{Error, Result};
use super::score::{Score, Scores};
use super::game_scorer::*;

#[cfg(test)]
use super::builder::Builder;

use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Game {
    Starting(Cuts, Deck),
    Discarding(Scores, Player, Hands, Crib, Deck),
    Playing(Scores, Player, Hands, PlayState, Card, Crib),
    ScoringPone(Scores, Player, Hands, Card, Crib),
    ScoringDealer(Scores, Player, Hands, Card, Crib),
    ScoringCrib(Scores, Player, Hands, Card, Crib),
    Finished(Scores),
}

impl Game {
    pub fn new(players: &HashSet<Player>) -> Result<Game> {
        if players.len() > NUMBER_OF_PLAYERS_IN_GAME {
            return Err(Error::TooManyPlayers)
        }

        let make_cut = |(mut cuts, deck): (HashMap<Player, Card>, Deck), player: &Player| {
            let (cut, deck) = deck.cut();
            cuts.insert(*player, cut);
            (cuts, deck)
        };

        let deck = Deck::shuffled_pack();
        let (cuts, deck) = players.iter().fold((HashMap::new(), deck), make_cut);

        Ok(Game::Starting(cuts, deck))
    }

    pub fn players(&self) -> HashSet<Player> {
        match self {
            Game::Starting(cuts, _) => HashSet::from_iter(cuts.keys().cloned()),
            Game::Discarding(scores, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::Playing(scores, _, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::ScoringPone(scores, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::ScoringDealer(scores, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::ScoringCrib(scores, _, _, _, _) => HashSet::from_iter(scores.keys().cloned()),
            Game::Finished(scores) => HashSet::from_iter(scores.keys().cloned()),
        }
    }

    pub(crate) fn player_1_2(&self) -> (Player, Player) {
        let players = Vec::from_iter(self.players());
        (players[0], players[1])
    }

    pub fn deck(&self) -> Deck {
        match self {
            Game::Starting(_, deck) => deck.clone(),
            // Game::Discarding(_, _, _, _, deck) => deck.clone(),
            _ => unreachable!(),
        }
    }

    pub fn scores(&self) -> Scores {
        match self {
            Game::Starting(_, _) => unreachable!(),
            Game::Discarding(scores, _, _, _, _) => scores,
            Game::Playing(scores, _, _, _, _, _) => scores,
            Game::ScoringPone(scores, _, _, _, _) => scores,
            Game::ScoringDealer(scores, _, _, _, _) => scores,
            Game::ScoringCrib(scores, _, _, _, _) => scores,
            Game::Finished(scores) => scores,
        }.clone()
    }

    fn has_winner(&self) -> bool {
        let scores = self.scores();
        !scores.values()
            .filter(|s| s.value() >= WINNING_SCORE)
            .collect::<Vec<_>>()
            .is_empty()
    }

    pub fn start(&self) -> Result<Self> {
        match self {
            Game::Starting(cuts, _) => {
                let players = self.players();
                verify::players(&players)?;
                verify::different_cuts(cuts)?;
                let scores: Scores = Scores::from_iter(players.iter().map(|&p| (p, Score::default())));
                let mut cuts = cuts.iter();
                let Some((player1, cut1)) = cuts.next() else { unreachable!() };
                let Some((player2, cut2)) = cuts.next() else { unreachable!() };
                let dealer = if cut1.rank() < cut2.rank() { player1 } else { player2 };
                let deck = Deck::shuffled_pack();
                let (hands, deck) = deck.deal(&players);
                let crib = Crib::default();
                Ok(Game::Discarding(scores, *dealer, hands, crib, deck))
            },
            _ => Err(Error::ActionNotPermitted)
        }
    }

    pub fn redraw(&self) -> Result<Self> {
        match self {
            Game::Starting(cuts, _) => {
                let players = self.players();
                verify::players(&players)?;
                verify::same_cuts(cuts)?;
                Ok(Game::new(&self.players())?)
            },
            _ => Err(Error::ActionNotPermitted)
        }
    }

    pub fn discard(&self, player: Player, discards: &[Card]) -> Result<Self> {
        match self {
            Game::Discarding(scores, dealer, hands, crib, deck) => {
                let players = self.players();
                verify::player(player, &players)?;

                let mut hands = hands.clone();
                let mut hand = hands[&player].clone();
                verify::discards(discards, &hand)?;

                hand.remove_all(discards);
                hands.insert(player, hand);

                let mut crib = crib.clone();
                crib.add(discards);

                if crib.len() == CARDS_REQUIRED_IN_CRIB {
                    let (cut, _) = deck.cut();
                    let pone = self.pone();
                    let play_state = PlayState::new(Some(pone), &hands);
                    let game = Game::Playing(scores.clone(), *dealer, hands, play_state, cut, crib);
                    let score = GameScorer::his_heels_on_cut_pre_play(cut);
                    game.score_points(*dealer, score)
                } else {
                    Ok(Game::Discarding(scores.clone(), *dealer, hands, crib, deck.clone()))
                }
            },
            _ => Err(Error::ActionNotPermitted)
        }
    }

    pub fn dealer(&self) -> Player {
        match self {
            Game::Starting(_, _) => unreachable!(),
            Game::Discarding(_, dealer, _, _, _) => *dealer,
            Game::Playing(_, dealer, _, _, _, _) => *dealer,
            Game::ScoringPone(_, dealer, _, _, _) => *dealer,
            Game::ScoringDealer(_, dealer, _, _, _) => *dealer,
            Game::ScoringCrib(_, dealer, _, _, _) => *dealer,
            Game::Finished(_) => unreachable!(),
        }
    }

    pub fn pone(&self) -> Player {
        let (player1, player2) = self.player_1_2();
        if self.dealer() == player1 { player2 } else { player1 }
    }

    pub fn opponent(&self, player: Player) -> Player {
        assert!(self.players().contains(&player));
        let (player1, player2) = self.player_1_2();
        if player == player1 { player2 } else { player1 }
    }

    fn score_points(&self, player: Player, score: usize) -> Result<Self> {
        let update = |mut scores: Scores| {
            scores.insert(player, scores[&player].add(score));
            scores
        };

        let mut game = match self {
            Game::Starting(_, _) => unreachable!(),

            Game::Discarding(scores, dealer, hands, crib, deck) => 
                Game::Discarding(update(scores.clone()), *dealer, hands.clone(), crib.clone(), deck.clone()),

            Game::Playing(scores, dealer, hands, play_state, cut, crib) =>
                Game::Playing(update(scores.clone()), *dealer, hands.clone(), play_state.clone(), *cut, crib.clone()),

            Game::ScoringPone(scores, dealer, hands, cut, crib) =>
                Game::ScoringPone(update(scores.clone()), *dealer, hands.clone(), *cut, crib.clone()),

            Game::ScoringDealer(scores, dealer, hands, cut, crib) =>
                Game::ScoringDealer(update(scores.clone()), *dealer, hands.clone(), *cut, crib.clone()),

            Game::ScoringCrib(scores, dealer, hands, cut, crib) =>
                Game::ScoringCrib(update(scores.clone()), *dealer, hands.clone(), *cut, crib.clone()),

            Game::Finished(_) => self.clone(),
        };

        if game.has_winner() {
            let scores = game.scores();
            game = Game::Finished(scores)
        }

        Ok(game)
    }

    pub fn play(&self, player: Player, card: Card) -> Result<Game> {
        let mut game = self.clone();

        let players = game.players();
        verify::player(player, &players)?;
        
        match game {
            Game::Playing(ref mut scores, dealer, ref mut hands, ref mut play_state, cut, ref mut crib) => {
                let hand = hands.get_mut(&player).unwrap();
                let legal_plays = play_state.legal_plays(player)?;
                verify::card(card, &hand.cards())?;
                verify::card(card, &legal_plays.cards()).map_err(|_| Error::CannotPlayCard)?;

                hand.remove(card);
                play_state.play(card);
                let score_current_play = GameScorer::current_play(play_state);
                let score_end_of_play = GameScorer::end_of_play(play_state);

                let all_plays_finished = play_state.are_plays_finished();
                let end_of_play = play_state.target_reached() || all_plays_finished;

                if end_of_play {
                    play_state.start_new_play();
                }
                
                game = if all_plays_finished {
                    let hands = play_state.finish_plays();
                    Game::Playing(scores.clone(), dealer, hands.clone(), play_state.clone(), cut, crib.clone())
                } else {
                    Game::Playing(scores.clone(), dealer, hands.clone(), play_state.clone(), cut, crib.clone())
                };

                game.score_points(player, score_current_play + score_end_of_play)
            },
            _ => Err(Error::ActionNotPermitted),
        }
    }

    pub(crate) fn pass(&self, player: Player) -> Result<Game> {
        let mut game = self.clone();
        let players = game.players();
        verify::player(player, &players)?;
        
        let pone = game.pone();

        match game {
            Game::Playing(ref mut scores, dealer, ref mut hands, ref mut play_state, cut, ref mut crib) => {
                let legal_plays = play_state.legal_plays(player)?;
                verify::no_legal_plays(&legal_plays.cards())?;

                play_state.pass();

                let mut score = 0;

                if play_state.pass_count() == NUMBER_OF_PLAYERS_IN_GAME {
                    score += GameScorer::end_of_play(play_state);                    
                    play_state.start_new_play();                    
                }

                game = if play_state.are_plays_finished() {
                    let hands = play_state.finish_plays();
                    score += GameScorer::hand(&hands[&pone], cut);
                    Game::Playing(scores.clone(), dealer, hands, play_state.clone(), cut, crib.clone())
                } else {
                    Game::Playing(scores.clone(), dealer, hands.clone(), play_state.clone(), cut, crib.clone())
                };

                game.score_points(player, score)
            },
            _ => Err(Error::ActionNotPermitted),
        }
    }

    pub(crate) fn score_pone(&self) -> Result<Game> {
        let mut game = self.clone();
        let pone = game.pone();
        
        match game {
            Game::Playing(ref mut scores, dealer, hands, play_state, cut, crib) => {
                verify::ready_to_score_pone(&play_state)?;
                game = Game::ScoringPone(scores.clone(), dealer, hands.clone(), cut, crib.clone());
                let score = GameScorer::hand(&hands[&pone], cut);
                game.score_points(pone, score)
            },
            _ => Err(Error::ActionNotPermitted),
        }
    }

    pub(crate) fn score_dealer(&self) -> Result<Game> {
        let mut game = self.clone();
        match game {
            Game::ScoringPone(ref mut scores, dealer, hands, cut, crib) => {
                game = Game::ScoringDealer(scores.clone(), dealer, hands.clone(), cut, crib.clone());
                let score = GameScorer::hand(&hands[&dealer], cut);
                game.score_points(dealer, score)
            },
            _ => Err(Error::ActionNotPermitted),
        }
    }

    pub(crate) fn score_crib(&self) -> Result<Game> {
        let mut game = self.clone();
        match game {
            Game::ScoringDealer(ref mut scores, dealer, hands, cut, crib) => {
                game = Game::ScoringCrib(scores.clone(), dealer, hands.clone(), cut, crib.clone());
                let score = GameScorer::crib(&crib, cut);
                game.score_points(dealer, score)
            },
            _ => Err(Error::ActionNotPermitted),
        }
    }

    pub(crate) fn deal_next_hands(&self) -> Result<Game> {
        match self {
            Game::ScoringCrib(scores, dealer, _, _, _) => {
                let players = self.players();
                let deck = Deck::shuffled_pack();
                let (hands, deck) = deck.deal(&players);
                let crib = Crib::default();
                Ok(Game::Discarding(scores.clone(), self.pone(), hands, crib, deck))
            },
            _ => Err(Error::ActionNotPermitted),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::Starting(cuts, deck) =>
                write!(f, "Starting(Cuts({}), Deck({}))", format_hashmap(cuts), deck),

            Game::Discarding(scores, dealer, hands, crib, deck) =>
                write!(f, "Discarding(Scores({}), Dealer({}), Hands({}), Crib({}), Deck({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), crib, deck),

            Game::Playing(scores, dealer, hands, play_state, cut, crib) =>
                write!(f, "Playing(Scores({}), Dealer({}), Hands({}), PlayState({}), Cut({})), Crib({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), play_state, cut, crib),

            Game::ScoringPone(scores, dealer, hands, cut, crib) =>
                write!(f, "ScoringPone(Scores({}), Dealer({}), Hands({}), Cut({})), Crib({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), cut, crib),

            Game::ScoringDealer(scores, dealer, hands, cut, crib) =>
                write!(f, "ScoringDealer(Scores({}), Dealer({}), Hands({}), Cut({})), Crib({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), cut, crib),
    
            Game::ScoringCrib(scores, dealer, hands, cut, crib) =>
                write!(f, "ScoringCrib(Scores({}), Dealer({}), Hands({}), Cut({})), Crib({}))",
                    format_hashmap(scores), dealer, format_hashmap(hands), cut, crib),
        
            Game::Finished(scores) =>
                write!(f, "Finished(Scores({}))", format_hashmap(scores)),
        }
    }
}

mod verify {
    use super::*;

    pub(crate) fn players(players: &HashSet<Player>) -> Result<()> {
        if players.len() != NUMBER_OF_PLAYERS_IN_GAME {
            Err(Error::NotEnoughPlayers)
        } else {
            Ok(())
        }
    }

    pub(crate) fn player(player: Player, players: &HashSet<Player>) -> Result<()> {
        if !players.contains(&player) {
            Err(Error::InvalidPlayer(player))
        } else {
            Ok(())
        }
    }

    pub(crate) fn different_cuts(cuts: &HashMap<Player, Card>) -> Result<()> {
        let cuts: HashSet<Rank> = HashSet::from_iter(cuts.values().map(|c| c.rank()));
        if cuts.len() != NUMBER_OF_PLAYERS_IN_GAME {
            Err(Error::CutForStartUndecided)
        } else {
            Ok(())
        }
    }

    pub(crate) fn same_cuts(cuts: &HashMap<Player, Card>) -> Result<()> {
        let cuts: HashSet<Rank> = HashSet::from_iter(cuts.values().map(|c| c.rank()));
        if cuts.len() == NUMBER_OF_PLAYERS_IN_GAME {
            Err(Error::CutForStartDecided)
        } else {
            Ok(())
        }
    }

    pub(crate) fn discards(discards: &[Card], hand: &Hand) -> Result<()> {
        for discard in discards {
            verify::card(*discard, &hand.cards())?
        }

        if hand.len() - discards.len() < CARDS_KEPT_PER_HAND {
            Err(Error::TooManyDiscards)
        } else {
            Ok(())
        }
    }

    pub(crate) fn card(card: Card, cards: &[Card]) -> Result<()> {
        if !cards.contains(&card) {
            Err(Error::InvalidCard(card))
        } else {
            Ok(())
        }
    }

    pub(crate) fn no_legal_plays(cards: &[Card]) -> Result<()> {
        if cards.is_empty() {
            Ok(())
        } else {
            Err(Error::CannotPass)
        }
    }

    pub(crate) fn ready_to_score_pone(play_state: &PlayState) -> Result<()> {
        if play_state.next_to_play().is_none() {
            Ok(())
        } else {
            Err(Error::CannotScorePone)
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::card::{Cut, Face};
    use crate::domain::plays::Play;

    #[test]
    fn play_with_zero_one_or_two_players() {
        for n in 0..=2 {
            let builder = Builder::new(n);
            let players = builder.players.clone();
            let game = builder.as_new().ok().unwrap();
            assert_eq!(game.players().len(), n);
            for player in players.into_iter() {
                assert!(game.players().contains(&player))
            }
        }
    }

    #[test]
    fn fail_to_play_with_more_than_two_players() {
        let builder = Builder::new(3);
        let error = builder.as_new().err().unwrap();
        assert_eq!(error, Error::TooManyPlayers);
    }

    #[test]
    fn use_a_standard_pack_of_cards() {
        let builder = Builder::new(2);
        let game = builder.as_new().ok().unwrap();
        let _deck = game.deck();
        assert!(true)
    }

    #[test]
    fn start_game_with_lowest_cut_as_dealer() {
        for (expected_dealer, cuts) in vec![(0, "ASKS"), (1, "KSAS")] {
            let builder = Builder::new(2)
                .with_cuts(cuts);
            let players = builder.players.clone();
            let game = builder.as_starting();
            let game = game.start().ok().unwrap();
            let Game::Discarding(_, dealer, _, _, _) = game else { panic!("Unexpected state") };
            assert_eq!(dealer, players[expected_dealer]);
        }
    }

    #[test]
    fn fail_to_start_game_if_cuts_are_the_same_value() {
        let game = Builder::new(2)
            .with_cuts("ASAC")
            .as_starting();
        let error = game.start().err().unwrap();
        assert_eq!(error, Error::CutForStartUndecided);
    }

    #[test]
    fn fail_to_start_game_if_not_enough_players() {
        let game = Builder::new(2)
            .with_cuts("AS")
            .as_starting();
        let error = game.start().err().unwrap();
        assert_eq!(error, Error::NotEnoughPlayers);
    }

    #[test]
    fn redraw_if_cuts_are_same_value() {
        let game = Builder::new(2)
            .with_cuts("ASAC")
            .as_starting();
        let game = game.redraw().ok().unwrap();
        let Game::Starting(ref _cuts, _) = game else { panic!("Unexpected state") };
        assert!(true)
    }

    #[test]
    fn fail_to_redraw_if_cuts_are_not_the_same_value() {
        let game = Builder::new(2)
            .with_cuts("ASKS")
            .as_starting();
        let error = game.redraw().err().unwrap();
        assert_eq!(error, Error::CutForStartDecided);
    }

    #[test]
    fn deal_six_cards_per_player() {
        let game = Builder::new(2)
            .with_cuts("ASKS")
            .as_starting();

        let game = game.start().ok().unwrap();
        let players = game.players();
        assert_eq!(players.len(), 2);

        let Game::Discarding(_, _, hands, _, _) = game else { panic!("Unexpected state") };

        players
            .iter()
            .for_each(|p| assert_eq!(hands[p].cards().len(), CARDS_DEALT_PER_HAND));
    }

    #[test]
    fn deal_when_draw_decided() {
        let game = Builder::new(2)
            .with_cuts("ASKS")
            .as_starting();
        let game = game.start().ok().unwrap();
        let players = game.players();

        let Game::Discarding(scores, _, hands, crib, deck) = game  else { panic!("Unexpected state") };
        
        players.iter().for_each(|p| {
            assert_eq!(scores[p].back_peg(), 0);
            assert_eq!(scores[p].front_peg(), 0);
        });
        
        players.iter().for_each(|p| {
            assert_eq!(hands[p].cards().len(), CARDS_DEALT_PER_HAND);
        });

        assert_eq!(crib.cards().len(), 0);
        assert_eq!(deck.cards().len(), 52 - (NUMBER_OF_PLAYERS_IN_GAME * CARDS_DEALT_PER_HAND));
    }

    #[test]
    fn redeal_after_crib_scored() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .with_crib("AHADASTD")
            .as_scoring_crib();
        let pone0 = game0.pone();
        let Game::ScoringCrib(scores0, dealer0, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.deal_next_hands().ok().unwrap();
        let pone1 = game1.pone();
        let Game::Discarding(scores1, dealer1, hands1, crib1, deck1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1, scores0);
        assert_eq!(dealer1, pone0);
        assert_eq!(pone1, dealer0);
        assert_eq!(hands1[&dealer1].len(), 6);
        assert_eq!(hands1[&pone1].len(), 6);
        assert!(crib1.is_empty());
        assert_eq!(deck1.len(), 40);
    }

    #[test]
    fn fail_redeal_when_crib_not_scored() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .with_crib("AHADASTD")
            .as_scoring_dealer();

        let error = game0.deal_next_hands().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);
    }

    #[test]
    fn player_can_discard_one_held_card_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, opponent) = game.player_1_2();

        let Game::Discarding(scores0, dealer0, hands0, _, deck0) = game.clone()  else { panic!("Unexpected state") };
        let player_hand0 = hands0[&player].clone();
        let player_discard = player_hand0.get(&[0]);

        let opponent_hand0 = hands0[&opponent].clone();

        let game = game.discard(player, &player_discard).ok().unwrap();
        let Game::Discarding(scores1, dealer1, hands1, crib1, deck1) = game  else { panic!("Unexpected state") };

        let player_hand1 = hands1[&player].clone();
        let opponent_hand1 = hands1[&opponent].clone();

        assert_eq!(scores1, scores0);
        assert_eq!(dealer1, dealer0);
        assert!(player_hand1.contains_none(&player_discard));
        assert!(crib1.contains_all(&player_discard));
        assert_eq!(opponent_hand1, opponent_hand0);
        assert_eq!(deck1, deck0);
    }

    #[test]
    fn player_can_discard_two_held_cards_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, opponent) = game.player_1_2();

        let Game::Discarding(scores0, dealer0, hands0, _, deck0) = game.clone()  else { panic!("Unexpected state") };
        let player_hand0 = hands0[&player].clone();
        let player_discard = player_hand0.get(&[0, 1]);

        let opponent_hand0 = hands0[&opponent].clone();

        let game = game.discard(player, &player_discard).ok().unwrap();
        let Game::Discarding(scores1, dealer1, hands1, crib1, deck1) = game  else { panic!("Unexpected state") };

        let player_hand1 = hands1[&player].clone();
        let opponent_hand1 = hands1[&opponent].clone();

        assert_eq!(scores1, scores0);
        assert_eq!(dealer1, dealer0);
        assert!(player_hand1.contains_none(&player_discard));
        assert!(crib1.contains_all(&player_discard));
        assert_eq!(opponent_hand1, opponent_hand0);
        assert_eq!(deck1, deck0);
    }

    #[test]
    fn player_cannot_discard_more_then_two_held_cards_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, _) = game.player_1_2();

        let Game::Discarding(_, _, hands0, _, _) = game.clone()  else { panic!("Unexpected state") };
        let hand0 = hands0[&player].clone();
        let discard = hand0.get(&[0, 1, 2]);

        let error = game.discard(player, &discard).err().unwrap();
        assert_eq!(error, Error::TooManyDiscards);
    }

    #[test]
    fn player_cannot_discard_non_held_cards_to_the_crib() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();

        let (player, opponent) = game.player_1_2();

        let Game::Discarding(_, _, hands0, _, _) = game.clone()  else { panic!("Unexpected state") };
        let hand0 = hands0[&opponent].clone();
        let discard = hand0.get(&[0, 1]);

        let Error::InvalidCard(_) = game.discard(player, &discard).err().unwrap() else { panic!("Unexpected error") };
    }

    #[test]
    fn cannot_discard_when_player_not_participating() {
        let game = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();
        
        let (player, _) = game.player_1_2();

        let Game::Discarding(_, _, hands0, _, _) = game.clone()  else { panic!("Unexpected state") };
        let hand0 = hands0[&player].clone();
        let discard = hand0.get(&[0, 1]);

        let non_player = Player::new();
        let error = game.discard(non_player, &discard).err().unwrap();
        assert_eq!(error, Error::InvalidPlayer(non_player));
    }

    fn after_discards_common_tests() -> (Scores, Scores, Cut, Player, Player) {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
            .as_discarding();
        
        let (player, opponent) = game0.player_1_2();

        let Game::Discarding(scores0, dealer0, hands0, _, deck0) = game0.clone()  else { panic!("Unexpected state") };
        let pone = game0.pone().clone();

        let player_hand0 = hands0[&player].clone();
        let player_discard = player_hand0.get(&[0, 1]);

        let opponent_hand0 = hands0[&opponent].clone();
        let opponent_discard = opponent_hand0.get(&[0, 1]);

        let game1 = game0.discard(player, &player_discard).ok().unwrap();
        let game1 = game1.discard(opponent, &opponent_discard).ok().unwrap();
        let Game::Playing(scores1, dealer1, hands1, play_state1, cut1, crib1) = game1 else { panic!("Unexpected state") };

        let hand1 = hands1[&player].clone();
        let opponent_hand1 = hands1[&opponent].clone();

        assert_eq!(dealer1, dealer0);
        assert!(hand1.contains_none(&player_discard));
        assert!(crib1.contains_all(&player_discard));
        assert!(opponent_hand1.contains_none(&opponent_discard));
        assert!(crib1.contains_all(&opponent_discard));
        assert!(deck0.contains(&cut1));
        assert_eq!(crib1.len(), CARDS_REQUIRED_IN_CRIB);
        assert_eq!(play_state1.legal_plays(pone).ok().unwrap(), hands1[&pone]);
        assert_eq!(play_state1.legal_plays(dealer0).err().unwrap(), Error::CannotPlay);
        assert_eq!(play_state1.pass_count(), 0);
        assert_eq!(play_state1.current_plays(), vec![]);
        assert_eq!(play_state1.previous_plays(), vec![]);

        (scores0, scores1, cut1, dealer1, pone)
    }

    #[test]
    fn start_the_play_after_discards() {
        let (scores0, scores1, cut, dealer, pone) = after_discards_common_tests();
        if cut.face() == Face::Jack {
            assert_eq!(scores0[&dealer].add(2), scores1[&dealer]);
            assert_eq!(scores0[&pone], scores1[&pone]);
        } else {
            assert_eq!(scores0, scores1)
        }
    }

    #[test]
    fn score_his_heels_when_jack_cut_after_discards() {
        loop {
            let (scores0, scores1, cut, dealer, pone) = after_discards_common_tests();
            if cut.face() == Face::Jack {
                assert_eq!(scores0[&dealer].add(2), scores1[&dealer]);
                assert_eq!(scores0[&pone], scores1[&pone]);
                break;
            }
        }
        assert!(true)
    }
    
    #[test]
    fn accept_valid_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(Some(1));
        let pone0 = game0.pone();
        let Game::Playing(scores0, dealer0, hands0, play_state0, cut0, crib0) = game0.clone() else { panic!("Unexpected state") };
        let dealer_hand0 = hands0[&dealer0].clone();
        let dealer_score0 = scores0[&dealer0].clone();
        let pone_score0 = scores0[&pone0].clone();

        assert_eq!(play_state0.legal_plays(dealer0).err().unwrap(), Error::CannotPlay);
        assert_eq!(play_state0.legal_plays(pone0).ok().unwrap().cards(), Hand::from("4S").cards());

        let game1 = game0.play(pone0, Card::from("4S")).ok().unwrap();
        let pone1 = game1.pone();

        let Game::Playing(scores1, dealer1, hands1, play_state1, cut1, crib1) = game1 else { panic!("Unexpected state") };
        let dealer_hand1 = hands1[&dealer1].clone();
        let pone_hand1 = hands1[&pone1].clone();
        let dealer_score1 = scores1[&dealer1].clone();
        let pone_score1 = scores1[&pone1].clone();

        assert_eq!(dealer_score1, dealer_score0);
        assert_eq!(pone_score1, pone_score0);
        assert_eq!(dealer1, dealer0);
        assert_eq!(dealer_hand1, dealer_hand0);
        assert_eq!(pone_hand1, Hand::default());
        assert_eq!(play_state1.next_to_play(), Some(dealer1));
        assert_eq!(play_state1.legal_plays(dealer1).ok().unwrap(), dealer_hand1);
        assert_eq!(cut1, cut0);
        assert_eq!(crib1, crib0);
    }

    #[test]
    fn cannot_play_when_player_not_participating() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(Some(1));

        let non_player = Player::new();
        let error = game0.play(non_player, Card::from("4S")).err().unwrap();
        assert_eq!(error, Error::InvalidPlayer(non_player));
    }

    #[test]
    fn cannot_play_when_unheld_card() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(Some(1));
        let pone0 = game0.pone();
        let card = Card::from("9S");
        let error = game0.play(pone0, card).err().unwrap();
        assert_eq!(error, Error::InvalidCard(card));
    }

    #[test]
    fn cannot_play_when_not_their_turn() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .as_playing(Some(1));
        let dealer0 = game0.dealer();
        let card = Card::from("9S");
        let error = game0.play(dealer0, card).err().unwrap();
        assert_eq!(error, Error::CannotPlay);
    }

    #[test]
    fn cannot_play_when_play_exceeds_target() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9S", "4S")
            .with_cut("AS")
            .with_current_plays(&vec![(0, "KH"), (0, "KC"), (0, "KD")])
            .as_playing(Some(1));
        let pone0 = game0.pone();
        let Game::Playing(_, _, _, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        assert_eq!(play_state0.legal_plays(pone0).ok().unwrap().cards(), Hand::from("").cards());

        let error = game0.play(pone0, Card::from("4S")).err().unwrap();
        assert_eq!(error, Error::CannotPlayCard)
    }

    #[test]
    fn score_play_when_target_not_reached_mid_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("5S", "5H")
            .with_cut("AS")
            .with_current_plays(&vec![(0, "TH")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, _, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };
        let score0_pone = scores0[&pone];

        let game1 = game0.play(pone, Card::from("5H")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, play_state1, _, _) = game1 else { panic!("Unexpected state") };
        let score1_pone = scores1[&pone];
    
        assert_eq!(score1_pone, score0_pone.add(2));
        assert_eq!(play_state1.next_to_play(), Some(dealer1));
    }

    #[test]
    fn score_play_when_target_not_reached_end_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("QS", "2H")
            .with_cut("QC")
            .with_current_plays(&vec![(0, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
            .as_playing(Some(1));

        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };
        let score0_pone = scores0[&pone];
        let score0_dealer = scores0[&dealer0];

        let game1 = game0.play(pone, Card::from("2H")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, play_state, _, _) = game1.clone() else { panic!("Unexpected state") };
        let score1_pone = scores1[&pone];
        let score1_dealer = scores1[&dealer1];

        assert_eq!(score1_pone, score0_pone.add(1));
        assert_eq!(score1_dealer, score0_dealer);
        assert_eq!(play_state.next_to_play(), Some(dealer1));
    }

    #[test]
    fn score_play_when_target_not_reached_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 120)
            .with_hands("AH", "5H")
            .with_cut("QC")
            .with_current_plays(&vec![(0, "JH")])
            .with_previous_plays(&vec![(0, "9H"), (0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, _, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };
        let score0_pone = scores0[&pone];

        let game1 = game0.play(pone, Card::from("5H")).ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };
        let score1_pone = scores1[&pone];
    
        assert_eq!(score1_pone, score0_pone.add(2));
    }

    #[test]
    fn score_play_when_target_reached_mid_play() {
        let card = Card::from("AH");
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("9H", "AH")
            .with_cut("KC")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(1, "2S"), (1, "QS"), (1, "6S")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, card).ok().unwrap();
        let Game::Playing(scores1, dealer1, hands1, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&pone], scores0[&pone].add(2));
        assert_eq!(dealer1, dealer0);
        assert!(!hands1[&pone].contains(&card));
        assert_eq!(play_state1.next_to_play(), Some(dealer0));
        assert!(play_state1.current_plays().is_empty());
        for p in play_state0.current_plays().into_iter() {
            assert!(play_state1.previous_plays().contains(&p))
        }
    }

    #[test]
    fn score_play_when_target_reached_end_play() {
        let card = Card::from("AH");
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_hands("QC", "AH")
            .with_cut("KC")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(1, "2S"), (1, "QS"), (1, "6S")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, cut0, crib0) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, card).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, play_state, cut1, crib1) = game1.clone() else { panic!("Unexpected state") };
        
        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&pone], scores0[&pone].add(2));
        assert_eq!(play_state.next_to_play(), Some(dealer1));
        assert_eq!(cut1, cut0);
        assert_eq!(crib1, crib0);
    }

    #[test]
    fn score_play_when_target_reached_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 120)
            .with_hands("QC", "AH")
            .with_cut("KC")
            .with_current_plays(&vec![(0, "TH"), (1, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(1, "9H"), (1, "5S"), (0, "6S")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("AH")).ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&pone], scores0[&pone].add(2));
        assert_eq!(scores1[&dealer0], scores0[&dealer0]);
    }

    #[test]
    fn score_play_when_plays_finished_and_game_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 120)
            .with_hands("", "AH")
            .with_cut("KC")
            .with_current_plays(&vec![(0, "8H"), (1, "JH"), (0, "QH")])
            .with_previous_plays(&vec![(1, "9H"), (0, "4S"), (1, "5S"), (0, "6S")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("AH")).ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&pone], scores0[&pone].add(1));
        assert_eq!(scores1[&dealer0], scores0[&dealer0]);
    }

    #[test]
    fn accept_pass_when_pone_has_no_valid_card() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("AH", "KH")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "QH")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, hands0, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };
        let game1 = game0.pass(pone);
        let game1 = game1.ok().unwrap();
        let Game::Playing(scores1, dealer1, hands1, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1, scores0);
        assert_eq!(dealer1, dealer0);
        assert_eq!(hands1, hands0);
        assert_eq!(play_state1.next_to_play(), Some(dealer0));
        assert_eq!(play_state1.pass_count(), 1);
        assert_eq!(play_state1.current_plays(), play_state0.current_plays());
        assert_eq!(play_state1.previous_plays(), play_state0.previous_plays());
    }

    #[test]
    fn accept_pass_when_dealer_has_no_valid_card() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, hands0, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let game2 = game1.pass(dealer0).ok().unwrap();

        let Game::Playing(scores2, dealer2, hands2, play_state2, _, _) = game2.clone() else { panic!("Unexpected state") };

        assert_eq!(scores2[&pone], scores0[&pone]);
        assert_eq!(scores2[&dealer2], scores0[&dealer0].add(1));
        assert_eq!(dealer2, dealer0);
        assert_eq!(hands2, hands0);
        assert_eq!(play_state2.next_to_play(), Some(pone));
        assert_eq!(play_state2.pass_count(), 0);
        assert!(play_state2.current_plays().is_empty());
        for p in play_state0.current_plays().into_iter() {
            assert!(play_state2.previous_plays().contains(&p))
        }
    }

    #[test]
    fn cannot_pass_when_player_not_participating() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(Some(1));

        let non_player = Player::new();
        let error = game0.pass(non_player).err().unwrap();
        assert_eq!(error, Error::InvalidPlayer(non_player));
    }

    #[test]
    fn cannot_pass_when_valid_card_held() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("AH", "AS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (0, "8H")])
            .as_playing(Some(1));
        let pone = game0.pone();

        let error = game0.pass(pone).err().unwrap();
        assert_eq!(error, Error::CannotPass);
    }

    #[test]
    fn score_pass_when_both_players_passed_playing() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let game2 = game1.pass(dealer0).ok().unwrap();

        let Game::Playing(scores2, dealer2, _, _, _, _) = game2.clone() else { panic!("Unexpected state") };
        assert_eq!(scores2[&pone], scores0[&pone]);
        assert_eq!(scores2[&dealer2], scores0[&dealer0].add(1));
    }

    #[test]
    fn score_pass_when_both_players_passed_finished() {
        let game0 = Builder::new(2)
            .with_scores(120, 0)
            .with_cut("AS")
            .with_hands("KH", "KS")
            .with_current_plays(&vec![(0, "TH"), (0, "JH"), (1, "QH")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let game2 = game1.pass(dealer0).ok().unwrap();

        let Game::Finished(scores2) = game2.clone() else { panic!("Unexpected state") };
        assert_eq!(scores2[&pone], scores0[&pone]);
        assert_eq!(scores2[&dealer0], scores0[&dealer0].add(1));
    }

    #[test]
    fn swap_player_after_pone_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("7H8H8D9C", "4S5STHJH")
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("4S")).ok().unwrap();
        let Game::Playing(_, dealer1, _, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(play_state1.next_to_play(), Some(dealer1));
    }

    #[test]
    fn swap_player_after_dealer_play() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("7H8H8D9C", "5STHJH")
            .with_current_plays(&vec![(1, "4S")])
            .as_playing(Some(0));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(dealer0, Card::from("9C")).ok().unwrap();
        let Game::Playing(_, dealer1, _, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(play_state1.next_to_play(), Some(pone));
    }

    #[test]
    fn swap_player_after_pone_pass() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("8H8D", "5SJH")
            .with_current_plays(&vec![(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let Game::Playing(_, dealer1, _, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(play_state1.next_to_play(), Some(dealer1));
    }

    #[test]
    fn swap_player_after_dealer_pass() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("7H8H8D", "4S5S")
            .with_current_plays(&vec![(1, "JH"), (0, "9C"), (1, "TH")])
            .as_playing(Some(0));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(dealer0).ok().unwrap();
        let Game::Playing(_, dealer1, _, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(play_state1.next_to_play(), Some(pone));
    }

    #[test]
    fn reset_play_after_pone_then_dealer_pass() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("8H8D", "5SJH")
            .with_current_plays(&vec![(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(pone).ok().unwrap();
        let game1 = game1.pass(dealer0).ok().unwrap();
        let Game::Playing(_, dealer1, _, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(play_state1.next_to_play(), Some(pone));
        assert_eq!(play_state1.previous_plays(), play_state0.current_plays());
        assert!(play_state1.current_plays().is_empty());
        assert_eq!(play_state1.pass_count(), 0);
    }

    #[test]
    fn reset_play_after_after_dealer_then_pone_pass() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("7H8H8D", "4S5S")
            .with_current_plays(&vec![(1, "JH"), (0, "9C"), (1, "TH")])
            .as_playing(Some(0));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.pass(dealer0).ok().unwrap();
        let game1 = game1.pass(pone).ok().unwrap();
        let Game::Playing(_, dealer1, _, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(play_state1.next_to_play(), Some(dealer1));
        assert_eq!(play_state1.previous_plays(), play_state0.current_plays());
        assert!(play_state1.current_plays().is_empty());
        assert_eq!(play_state1.pass_count(), 0);
    }

    #[test]
    fn reset_play_after_exact_target_reached() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("7H8H8D", "5STH")
            .with_current_plays(&vec![(1, "JH"), (0, "9C"), (1, "4S")])
            .as_playing(Some(0));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, play_state0, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(dealer0, Card::from("8H")).ok().unwrap();
        let Game::Playing(_, dealer1, _, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        let last_play = Play::new(dealer0, Card::from("8H"));
        let mut expected_current_plays = play_state0.current_plays().clone();
        expected_current_plays.push(last_play);

        assert_eq!(dealer1, dealer0);
        assert_eq!(play_state1.next_to_play(), Some(pone));
        assert_eq!(play_state1.previous_plays(), expected_current_plays);
        assert!(play_state1.current_plays().is_empty());
        assert_eq!(play_state1.pass_count(), 0);
    }

    #[test]
    fn no_next_to_play_when_all_plays_completed() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("7H", "")
            .with_current_plays(&vec![(1, "TH"), (0, "8D"), (1, "5S")])
            .with_previous_plays(&vec![(1, "JH"), (0, "9C"), (1, "4S"), (0, "8H")])
            .as_playing(Some(0));
        let pone = game0.pone();
        let Game::Playing(_, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(dealer0, Card::from("7H")).ok().unwrap();
        let Game::Playing(_, dealer1, hands1, play_state1, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(hands1[&dealer1], Hand::from("9C8H8D7H"));
        assert_eq!(hands1[&pone], Hand::from("JH4STH5S"));
        assert_eq!(play_state1.next_to_play(), None);
        assert!(play_state1.current_plays().is_empty());
        assert!(play_state1.previous_plays().is_empty());
        assert_eq!(play_state1.pass_count(), 0);

    }
    
    #[test]
    fn score_play_points_for_fifteens() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "8D")
            .with_current_plays(&vec![(0, "7D")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(2));
    }

    #[test]
    fn score_play_points_for_pair() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "8D")
            .with_current_plays(&vec![(0, "8S")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(2));
    }

    #[test]
    fn score_play_points_for_triplet() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "8DAH")
            .with_current_plays(&vec![(1, "8C"), (0, "8S")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(6));
    }

    #[test]
    fn score_play_points_for_quartet() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "7DAH")
            .with_current_plays(&vec![(1, "7C"), (0, "7S"), (0, "7H")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("7D")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(12));
    }

    #[test]
    fn score_play_points_for_run() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("KH", "AS")
            .with_current_plays(&vec![(1, "2D"), (0, "3H")])
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("AS")).ok().unwrap();
        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(3));
    }

    #[test]
    fn score_play_points_for_run_edge_case_1() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("5H7H6H", "AH8S7S")
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("8S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("7H")).ok().unwrap();
        let game1 = game1.play(pone, Card::from("7S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("6H")).ok().unwrap();

        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0].add(2));
        assert_eq!(scores1[&pone], scores0[&pone].add(2));
    }

    #[test]
    fn score_play_points_for_run_edge_case_2() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("AS")
            .with_hands("5H7H6H", "AH9S8S")
            .as_playing(Some(1));
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.play(pone, Card::from("9S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("6H")).ok().unwrap();
        let game1 = game1.play(pone, Card::from("8S")).ok().unwrap();
        let game1 = game1.play(dealer0, Card::from("7H")).ok().unwrap();

        let Game::Playing(scores1, dealer1, _, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0].add(2).add(4));
        assert_eq!(scores1[&pone], scores0[&pone]);
    }

    #[test]
    fn score_pone_when_plays_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JHKS5HTH")
            .as_playing(None);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer0, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.score_pone().ok().unwrap();
        let Game::ScoringPone(scores1, dealer1, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(dealer1, dealer0);
        assert_eq!(scores1[&dealer1], scores0[&dealer0]);
        assert_eq!(scores1[&pone], scores0[&pone].add(7));
    }

    #[test]
    fn score_winning_pone_when_plays_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 115)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JHKS5HTH")
            .as_playing(None);
        let pone = game0.pone();
        let Game::Playing(scores0, dealer, _, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.score_pone().ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&dealer], scores0[&dealer]);
        assert_eq!(scores1[&pone], scores0[&pone].add(7));
    }

    #[test]
    fn fail_to_score_pone_when_plays_not_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .as_playing(Some(1));

        let error = game0.score_pone().err().unwrap();
        assert_eq!(error, Error::CannotScorePone);
    }

    #[test]
    fn fail_to_score_pone_when_already_scored_1() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .as_scoring_pone();

        let error = game0.score_pone().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);        
    }

    #[test]
    fn fail_to_score_pone_when_already_scored_2() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .as_scoring_dealer();

        let error = game0.score_pone().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);        
    }

    #[test]
    fn fail_to_score_pone_when_already_scored_3() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .as_scoring_crib();

        let error = game0.score_pone().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);           
    }

    #[test]
    fn fail_to_score_dealer_when_plays_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 115)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JHKS5HTH")
            .as_playing(None);

        let error = game0.score_dealer().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);       
    }

    #[test]
    fn fail_to_score_crib_when_plays_finished() {
        let game0 = Builder::new(2)
            .with_scores(0, 115)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JHKS5HTH")
            .as_playing(None);

        let error = game0.score_crib().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);       
    }

    #[test]
    fn score_dealer_after_pone_scored() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .as_scoring_pone();
        let pone = game0.pone();
        let Game::ScoringPone(scores0, dealer0, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.score_dealer().ok().unwrap();
        let Game::ScoringDealer(scores1, dealer1, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&dealer0], scores0[&dealer1].add(4));
        assert_eq!(scores1[&pone], scores0[&pone]);
    }

    #[test]
    fn score_winning_dealer_after_pone_scored() {
        let game0 = Builder::new(2)
            .with_scores(117, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .as_scoring_pone();
        let pone = game0.pone();
        let Game::ScoringPone(scores0, dealer, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.score_dealer().ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&dealer], scores0[&dealer].add(4));
        assert_eq!(scores1[&pone], scores0[&pone]);
    }

    #[test]
    fn fail_to_score_dealer_when_already_scored() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .as_scoring_dealer();

        let error = game0.score_dealer().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);
    }

    #[test]
    fn score_crib_after_dealer_scored() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .with_crib("AHADASTD")
            .as_scoring_dealer();
        let pone = game0.pone();
        let Game::ScoringDealer(scores0, dealer0, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.score_crib().ok().unwrap();
        let Game::ScoringCrib(scores1, dealer1, _, _, _) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&dealer0], scores0[&dealer1].add(12));
        assert_eq!(scores1[&pone], scores0[&pone]);
    }

    #[test]
    fn score_winning_crib_after_dealer_scored() {
        let game0 = Builder::new(2)
            .with_scores(110, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .with_crib("AHADASTD")
            .as_scoring_dealer();
        let pone = game0.pone();
        let Game::ScoringDealer(scores0, dealer, _, _, _) = game0.clone() else { panic!("Unexpected state") };

        let game1 = game0.score_crib().ok().unwrap();
        let Game::Finished(scores1) = game1.clone() else { panic!("Unexpected state") };

        assert_eq!(scores1[&dealer], scores0[&dealer].add(12));
        assert_eq!(scores1[&pone], scores0[&pone]);
    }

    #[test]
    fn fail_to_score_crib_when_already_scored() {
        let game0 = Builder::new(2)
            .with_scores(0, 0)
            .with_cut("4H")
            .with_hands("7H8CAC2C", "JCKS5HTH")
            .with_crib("AHADASTD")
            .as_scoring_crib();

        let error = game0.score_crib().err().unwrap();
        assert_eq!(error, Error::ActionNotPermitted);
    }

}   