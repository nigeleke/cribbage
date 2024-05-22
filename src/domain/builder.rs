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
            crib: Default::default(),
            cut: Default::default(),
            deck: Deck::shuffled_pack(),
        }
    }
}

impl Builder {
    pub fn with_players(mut self, count: usize) -> Self {
        let _ = (1..=count).map(|_| self.players.push(Player::new())).collect::<Vec<_>>();
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

    pub fn with_scores(mut self, scores: &[usize]) -> Self {
        let mut scores = Vec::from_iter(scores.iter().map(|s| Score::default().add(*s)));
        self.scores.append(&mut scores);
        self
    }

    pub fn with_hand(mut self, hand: &str) -> Self {
        let hand = Builder::cards(hand);
        self.deck.remove_all(&hand);
        self.hands.push(hand.into());
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
        self.deck.remove(&cut);
        self.cut = Some(cut);
        self
    }

    pub fn as_new(self) -> Result<Game> {
        let players = HashSet::from_iter(self.players.into_iter());
        Game::new(&players)
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

    fn merged<T>(&self, items: Vec<T>) -> HashMap<Player, T> {
        let players = self.players.clone();
        let zipped = players.into_iter().zip(items);
        zipped.collect()
    }

    fn cards(cards: &str) -> Vec<Card> {
        let cards = cards
            .chars()
            .collect::<Vec<_>>().chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|cid| Builder::card(&cid))
            .collect::<Vec<Card>>();
        Vec::from_iter(cards)
    }
    
    fn card(cid: &str) -> Card {
        Card::from(cid)
    }
    
    
}

