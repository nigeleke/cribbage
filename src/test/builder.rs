use crate::domain::*;
use crate::types::Player;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Builder {
    players: Vec<Player>,
    dealer: usize,
    cuts: Vec<Card>,
    scores: Vec<Score>,
    hands: Vec<Hand>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
    pass_count: usize,
    crib: Crib,
    cut: Option<Card>,
    deck: Deck,
}

impl Builder {
    pub fn new(player_count: usize) -> Self {
        Self {
            players: Vec::from_iter((0..player_count).map(|_| Player::new())),
            deck: Deck::shuffled_pack(),
            ..Default::default()
        }
    }

    pub fn players(&self) -> Vec<Player> {
        self.players.clone()
    }

    pub fn with_dealer(mut self, dealer: usize) -> Self {
        self.dealer = dealer;
        self
    }

    pub fn with_cuts(mut self, cuts: &str) -> Self {
        #[derive(Clone, Debug, PartialEq)]
        pub struct CutsType;
        pub type Cuts = Cards<CutsType>;

        let cuts = Cuts::from(cuts);
        self.deck.remove_all(cuts.as_ref());
        cuts.as_ref().iter().for_each(|c| self.cuts.push(*c));
        self
    }

    pub fn with_scores(mut self, score0: usize, score1: usize) -> Self {
        self.scores.push(Score::default().add(score0.into()));
        self.scores.push(Score::default().add(score1.into()));
        self
    }

    pub fn with_hands(mut self, hand0: &str, hand1: &str) -> Self {
        let mut add_hand = |hand: &str| {
            let hand = Hand::from(hand);
            self.deck.remove_all(hand.as_ref());
            self.hands.push(hand);
        };

        add_hand(hand0);
        add_hand(hand1);

        self
    }

    pub fn with_crib(mut self, crib: &str) -> Self {
        let crib = Crib::from(crib);
        self.deck.remove_all(crib.as_ref());
        self.crib = crib.into();
        self
    }

    pub fn with_cut(mut self, cut: &str) -> Self {
        let cut = Card::from(cut);
        self.deck.remove(cut);
        self.cut = Some(cut);
        self
    }

    pub fn with_current_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays.into_iter().map(|(p, c)| (self.players[*p], Card::from(*c)));
        let plays = plays.map(|(p, c)| Play::new(p, c));
        self.current_plays = Vec::from_iter(plays);
        self
    }

    pub fn with_previous_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays.into_iter().map(|(p, c)| (self.players[*p], Card::from(*c)));
        let plays = plays.map(|(p, c)| Play::new(p, c));
        self.previous_plays = Vec::from_iter(plays);
        self
    }

    pub fn with_pass(mut self) -> Self {
        self.pass_count += 1;
        self
    }

    pub fn as_new(self) -> Game {
        let players = HashSet::from_iter(self.players.into_iter());
        Game::new(&players).ok().unwrap()
    }

    pub fn as_starting(self) -> Game {
        let deck = self.deck.clone();
        let cuts = self.cuts.clone();
        let cuts = self.merged(cuts);
        Game::Starting(cuts, Deck::from(deck))
    }

    pub fn as_discarding(self) -> Game {
        let players = self.players.clone();
        let scores = self.scores.clone();
        let scores = self.merged(scores);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let crib = self.crib.clone();
        let deck = self.deck.clone();
        Game::Discarding(scores, players[self.dealer], hands, crib, deck)
    }

    pub fn as_playing(self, next_to_play: usize) -> Game {
        let players = self.players.clone();
        let player = players[next_to_play];
        let scores = self.scores.clone();
        let scores = self.merged(scores);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let mut play_state = PlayState::new(player, &hands);
        play_state
            .force_pass_count(self.pass_count);
        self.current_plays
            .iter()
            .for_each(|p| play_state.force_current_play(p.player(), p.card()));
        self.previous_plays
            .iter()
            .for_each(|p| play_state.force_previous_play(p.player(), p.card()));
        let cut = self.cut.unwrap();
        let crib = self.crib.clone();
        Game::Playing(scores, players[self.dealer], hands, play_state, cut, crib)
    }

    pub fn as_scoring_pone(self) -> Game {
        let players = self.players.clone();
        let scores = self.scores.clone();
        let scores = self.merged(scores);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let cut = self.cut.unwrap();
        let crib = self.crib.clone();
        Game::ScoringPone(scores, players[self.dealer], hands, cut, crib)

    }
    pub fn as_scoring_dealer(self) -> Game {
        let players = self.players.clone();
        let scores = self.scores.clone();
        let scores = self.merged(scores);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let cut = self.cut.unwrap();
        let crib = self.crib.clone();
        Game::ScoringDealer(scores, players[self.dealer], hands, cut, crib)
    }

    pub fn as_scoring_crib(self) -> Game {
        let players = self.players.clone();
        let scores = self.scores.clone();
        let scores = self.merged(scores);
        let hands = self.hands.clone();
        let hands = self.merged(hands);
        let cut = self.cut.unwrap();
        let crib = self.crib.clone();
        Game::ScoringCrib(scores, players[self.dealer], hands, cut, crib)
    }

    fn merged<T>(&self, items: Vec<T>) -> HashMap<Player, T> {
        let players = self.players.clone();
        let zipped = players.into_iter().zip(items);
        zipped.collect()
    }
    
}

