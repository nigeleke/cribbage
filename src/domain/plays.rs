use super::card::{Card, Value};
use super::cards::{Hand, Hands};
use super::format::{format_hashmap, format_vec};
use super::player::Player;
use super::prelude::PLAY_TARGET;
use super::result::{Error, Result};

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Play {
    player: Player,
    card: Card,
}

impl Play {
    fn new(player: Player, card: Card) -> Self {
        Self { player, card }
    }

    pub fn player(self) -> Player {
        self.player
    }

    pub fn card(self) -> Card {
        self.card
    }

    pub fn value(self) -> Value {
        self.card.value()
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.player, self.card)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PlayState {
    next_to_play: Player,
    legal_plays: Hands,
    pass_count: usize,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {
    pub(crate) fn new(next_to_play: Player, hands: &Hands) -> Self {
        Self {
            next_to_play,
            legal_plays: hands.clone(),
            pass_count: 0,
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
        }
    }

    pub(crate) fn running_total(&self) -> Value {
        let cards = Hand::from(self.current_plays
            .iter()
            .map(|p| p.card)
            .collect::<Vec<_>>());
        cards.value()
    }

    pub(crate) fn legal_plays(&self, player: Player) -> Result<Hand> {
        if player == self.next_to_play {
            Ok(self.legal_plays_unchecked(player))
        } else {
            Err(Error::CannotPlay)
        }
    }

    fn legal_plays_unchecked(&self, player: Player) -> Hand {
        let running_total = self.running_total();
        let legal_plays: Hand = self.legal_plays[&player].cards().into_iter()
            .filter(|c| running_total + c.value() <= PLAY_TARGET.into())
            .collect::<Vec<_>>().into();
        legal_plays
    }

    pub(crate) fn pass_count(&self) -> usize {
        self.pass_count
    }

    pub(crate) fn current_plays(&self) -> Vec<Play> {
        self.current_plays.clone()
    }

    pub(crate) fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }

    pub(crate) fn play(&mut self, card: Card) {
        let player = self.next_to_play;
        if self.pass_count() == 0 {
            self.make_opponent_next_player();
        }

        let legal_plays = &mut self.legal_plays;

        let legal_cards = legal_plays.get_mut(&player).unwrap();
        legal_cards.remove(card);

        let play = Play::new(player, card);
        self.current_plays.push(play);
    }

    pub(crate) fn pass(&mut self) {
        self.make_opponent_next_player();
        self.pass_count += 1;
    }

    fn make_opponent_next_player(&mut self) {
        let legal_plays = &mut self.legal_plays;

        let mut players = legal_plays.keys();
        let (player1, player2) = (players.next().unwrap(), players.next().unwrap());

        let player = self.next_to_play;
        let opponent = if player == *player1 { *player2 } else { *player1 };
        self.next_to_play = opponent;
    }

    pub(crate) fn start_new_play(&mut self) {
        self.previous_plays.append(&mut self.current_plays);
    }

    pub(crate) fn target_reached(&self) -> bool {
        self.running_total() == Value::from(PLAY_TARGET)
    }

    pub(crate) fn finished_plays(&self) -> bool {
        let legal_plays = &self.legal_plays;
        legal_plays.iter().all(|(_, hand)| hand.is_empty())
    }

    pub(crate) fn next_to_play(&self) -> Player {
        self.next_to_play
    }

    #[cfg(test)]
    pub(crate) fn force_current_play(&mut self, player: Player, card: Card) {
        self.current_plays.push(Play::new(player, card))
    }

    #[cfg(test)]
    pub(crate) fn force_previous_play(&mut self, player: Player, card: Card) {
        self.previous_plays.push(Play::new(player, card))
    }

    #[cfg(test)]
    pub(crate) fn force_pass_count(&mut self, n: usize) {
        self.pass_count = n;
    }

}

impl std::fmt::Display for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Next({}), Legal({}), Passes({}), Current({}), Previous({})",
            self.next_to_play,
            format_hashmap(&self.legal_plays),
            self.pass_count,
            format_vec(&self.current_plays),
            format_vec(&self.previous_plays)
        )
    }
}