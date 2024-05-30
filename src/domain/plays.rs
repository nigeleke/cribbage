use super::card::{Card, Value};
use super::cards::{Hand, Hands};
use super::format::{format_hashmap, format_vec};
use super::player::Player;
use super::prelude::PLAY_TARGET;
use super::result::{Error, Result};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Play {
    player: Player,
    card: Card,
}

impl Play {
    pub fn player(&self) -> Player {
        self.player
    }

    pub fn card(&self) -> Card {
        self.card
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.player, self.card)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PlayState {
    legal_plays: Hands,
    pass_count: usize,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {

    pub(crate) fn legal_plays(&self, player: Player) -> Result<Hand> {
        self.legal_plays.get(&player).cloned().ok_or(Error::NotYourPlay)
    }

    pub(crate) fn with_legal_plays_for_player_hand(&self, player: Player, hand: &Hand) -> Self {
        let running_total = self.running_total();
        let legal_plays: Hand = hand.cards().into_iter()
            .filter(|c| running_total + c.value() <= PLAY_TARGET.into())
            .collect::<Vec<_>>().into();
        let legal_plays: Hands = Hands::from_iter(vec![(player, legal_plays)]);
        Self {
            legal_plays,
            ..self.clone()
        }
    }

    pub(crate) fn running_total(&self) -> Value {
        let cards = Hand::from(self.current_plays
            .iter()
            .map(|p| p.card)
            .collect::<Vec<_>>());
        cards.value()
    }

    #[cfg(test)]
    pub(crate) fn pass_count(&self) -> usize {
        self.pass_count
    }

    pub(crate) fn current_plays(&self) -> Vec<Play> {
        self.current_plays.clone()
    }

    #[cfg(test)]
    pub(crate) fn force_current_play(&mut self, player: Player, card: Card) {
        self.current_plays.push(Play { player, card })
    }

    #[cfg(test)]
    pub(crate) fn force_previous_play(&mut self, player: Player, card: Card) {
        self.previous_plays.push(Play { player, card })
    }

    pub(crate) fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }

    pub(crate) fn play(&mut self, card: Card) {
        let (player, mut legal_cards) = self.legal_plays.clone().into_iter().next().unwrap();
        legal_cards.remove(card);
        self.legal_plays.insert(player, legal_cards);
        let play = Play { player, card };
        self.current_plays.push(play);
    }
}

impl std::fmt::Display for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Legal({}), Passes({}), Current({}), Previous({})",
            format_hashmap(&self.legal_plays),
            self.pass_count,
            format_vec(&self.current_plays),
            format_vec(&self.previous_plays)
        )
    }
}