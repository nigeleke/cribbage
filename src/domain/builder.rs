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
    deck: Vec<Card>,
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
            deck: Deck::shuffled_pack().cards(),
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
        let mut cuts = str_to_cards(cuts);
        self.remove_from_deck(&cuts);
        self.cuts.append(&mut cuts);
        self
    }

    pub fn with_scores(mut self, scores: &[usize]) -> Self {
        let mut scores = Vec::from_iter(scores.iter().map(|s| Score::default().add(*s)));
        self.scores.append(&mut scores);
        self
    }

    pub fn with_hand(mut self, hand: &str) -> Self {
        let hand = str_to_cards(hand);
        self.remove_from_deck(&hand);
        let hand = Hand::from(hand);
        self.hands.push(hand);
        self
    }

    pub fn with_crib(mut self, crib: &str) -> Self {
        let crib = str_to_cards(crib);
        self.remove_from_deck(&crib);
        let crib = Crib::from(crib);
        self.crib = crib;
        self
    }

    pub fn with_cut(mut self, cut: &str) -> Self {
        let cut = str_to_card(cut);
        self.remove_from_deck(&vec![cut]);
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

    fn remove_from_deck(&mut self, cards: &[Card]) {
        let deck = self.deck.clone();
        let deck = deck.into_iter().filter(|c| !cards.contains(c));
        println!("Left in deck {:?}", deck);
        self.deck = deck.collect();
    }

    fn merged<T>(self, items: Vec<T>) -> HashMap<Player, T> {
        let zipped = self.players.into_iter().zip(items);
        zipped.collect()
    }

    // pub fn as_discarding(self) -> Game::Discarding {
    //     Game::Discarding(self.players, players[self.dealer], self.hands, self.crib, self.cut)
    // }
}

fn str_to_cards(cards: &str) -> Vec<Card> {
    let cards = cards
        .chars()
        .collect::<Vec<_>>().chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .map(|cid| str_to_card(&cid))
        .collect::<Vec<Card>>();
    Vec::from_iter(cards)
}

fn str_to_card(cid: &str) -> Card {
    Card::from(cid)
}

