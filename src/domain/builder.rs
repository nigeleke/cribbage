use super::prelude::*;
use super::result::Result;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Builder {
    pub(crate) players: Vec<Player>,
    dealer: usize,
    cuts: Vec<Card>,
    scores: Vec<Score>,
    hands: Vec<Hand>,
    play_state: PlayState,
    crib: Crib,
    cut: Option<Card>,
    deck: Deck,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            players: Default::default(), 
            dealer: Default::default(), 
            cuts: Default::default(),
            scores: Default::default(),
            hands: Default::default(),
            play_state: Default::default(),
            crib: Default::default(),
            cut: Default::default(),
            deck: Deck::shuffled_pack(),
        }
    }
}

impl Builder {
    pub fn with_player(mut self) -> Self {
        self.players.push(Player::new());
        self
    }

    pub fn with_dealer(mut self, dealer: usize) -> Self {
        self.dealer = dealer;
        self
    }

    pub fn with_cuts(mut self, cuts: &str) -> Self {
        let mut cuts = Builder::cards(cuts);
        self.deck.remove_all(&cuts);
        self.cuts.append(&mut cuts);
        self
    }

    pub fn with_scores(mut self, score0: usize, score1: usize) -> Self {
        self.scores.push(Score::default().add(score0));
        self.scores.push(Score::default().add(score1));
        self
    }

    pub fn with_hands(mut self, hand0: &str, hand1: &str) -> Self {
        let mut add_hand = |hand: &str| {
            let hand = Hand::from(Builder::cards(hand));
            self.deck.remove_all(&hand.cards());
            self.hands.push(hand);
        };

        add_hand(hand0);
        add_hand(hand1);

        self
    }

    pub fn with_crib(mut self, crib: &str) -> Self {
        let crib = Builder::cards(crib);
        self.deck.remove_all(&crib);
        self.crib = crib.into();
        self
    }

    pub fn with_cut(mut self, cut: &str) -> Self {
        let cut = Builder::card(cut);
        self.deck.remove(cut);
        self.cut = Some(cut);
        self
    }

    pub fn with_current_plays(self, plays: &[(usize, &str)]) -> Self {
        let mut self_checked = self.force_two_players();
        let _ = plays.into_iter().for_each(|(p, c)| self_checked.play_state.force_current_play(self_checked.players[*p], Builder::card(*c)));
        self_checked
    }

    pub fn with_previous_plays(self, plays: &[(usize, &str)]) -> Self {
        let mut self_checked = self.force_two_players();
        let _ = plays.into_iter().for_each(|(p, c)| self_checked.play_state.force_previous_play(self_checked.players[*p], Builder::card(*c)));
        self_checked
    }

    fn force_two_players(mut self) -> Self {
        let n = self.players.len();
        (n..=2).for_each(|_| self.players.push(Player::new()));
        self
    }

    pub fn as_new(self) -> Result<Game> {
        let players = HashSet::from_iter(self.players.into_iter());
        Game::new(&players)
    }

    pub fn as_starting(self) -> Game {
        let self_checked = self.force_two_players();
        let deck = self_checked.deck.clone();
        let cuts = self_checked.cuts.clone();
        let cuts = self_checked.merged(cuts);
        Game::Starting(cuts, Deck::from(deck))
    }

    pub fn as_discarding(self) -> Game {
        let self_checked = self.force_two_players();
        let players = self_checked.players.clone();
        let scores = self_checked.scores.clone();
        let scores = self_checked.merged(scores);
        let hands = self_checked.hands.clone();
        let hands = self_checked.merged(hands);
        let crib = self_checked.crib.clone();
        let deck = self_checked.deck.clone();
        Game::Discarding(scores, players[self_checked.dealer], hands, crib, deck)
    }

    pub fn as_playing(self, next_to_play: usize) -> Game {
        let self_checked = self.force_two_players();
        let players = self_checked.players.clone();
        let player = players[next_to_play];
        let scores = self_checked.scores.clone();
        let scores = self_checked.merged(scores);
        let hands = self_checked.hands.clone();
        let hands = self_checked.merged(hands);
        let play_state = self_checked.play_state.with_legal_plays_for_player_hand(player, &self_checked.hands[next_to_play]);
        let cut = self_checked.cut.unwrap();
        let crib = self_checked.crib.clone();
        Game::Playing(scores, players[self_checked.dealer], hands, play_state, cut, crib)
    }

    fn merged<T>(&self, items: Vec<T>) -> HashMap<Player, T> {
        let players = self.players.clone();
        let zipped = players.into_iter().zip(items);
        zipped.collect()
    }

    pub fn cards(cards: &str) -> Vec<Card> {
        let cards = Self::card_chunks(cards)
            .iter()
            .map(|cid| Builder::card(&cid))
            .collect::<Vec<Card>>();
        Vec::from_iter(cards)
    }

    pub fn card_chunks(cards: &str) -> Vec<String> {
        cards
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
    }
    
    pub fn card(cid: &str) -> Card {
        Card::from(cid)
    }
    
    
}

