use super::player::Player;
use super::card::{Card, Value};
use super::cards::{Hand, Hands};
use super::prelude::PLAY_TARGET;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

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

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct PlayState {
    legal_plays: Hands,
    pass_count: usize,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {

    pub(crate) fn legal_plays(&self, player: &Player) -> Hand {
        self.legal_plays.get(player).unwrap_or(&Hand::default()).clone()
    }

    pub(crate) fn with_legal_plays_for_player_hand(&self, player: &Player, hand: &Hand) -> Self {
        let running_total = self.running_total();
        let legal_plays: Hand = hand.cards().into_iter()
            .filter(|c| running_total + c.value() <= PLAY_TARGET.into())
            .collect::<Vec<_>>().into();
        let legal_plays: Hands = Hands::from_iter(vec![(*player, legal_plays)]);
        Self {
            legal_plays,
            ..self.clone()
        }
    }

    fn running_total(&self) -> Value {
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

    pub(crate) fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }
}