use super::card::{Card, CardSlot, Cut};
use super::cards::{Crib, Cuts, Hands};
use super::plays::PlayState;
use super::role::{Dealer, Role};
use super::score::Scores;

use crate::domain::prelude::{Game as DomainGame, Player as DomainPlayer};

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Game {
    Starting(Cuts),
    Discarding(Scores, Hands, Crib, Dealer),
    Playing(Scores, Hands, PlayState, Cut, Crib, Dealer),
    ScoringPone(Scores, Hands, Cut, Crib),
// //     ScoringDealerCards(MyState, OpponentState, Cut, Crib),
// //     ScoringCrib(MyState, OpponentState, Cut, Crib),
// //     Finished(MyScore, OpponentScore),
}

impl Game {
    pub(crate) fn scores(&self) -> Scores {
        match self {
            Game::Starting(_) => Scores::new(),
            Game::Discarding(scores, _, _, _) => scores.clone(),
            Game::Playing(scores, _, _, _, _, _) => scores.clone(),
            Game::ScoringPone(scores, _, _, _) => scores.clone(),
        }
    }

    pub(crate) fn dealer(&self) -> Option<Role> {
        match self {
            Game::Starting(_) => None,
            Game::Discarding(_, _, _, dealer) => Some(dealer.clone()),
            Game::Playing(_, _, _, _, _, dealer) => Some(dealer.clone()),
            Game::ScoringPone(_, _, _, _) => unreachable!(),
        }
    }
}

impl From<(DomainGame, DomainPlayer)> for Game {
    fn from((game, player): (DomainGame, DomainPlayer)) -> Self {
        match game {
            DomainGame::Starting(cuts, _) => {
                let (player_cut, opponent_cut) = partition_for(player, &cuts);
                let cuts = merge(player_cut, opponent_cut);
                Game::Starting(cuts)
            },
            DomainGame::Discarding(scores, dealer, hands, crib, _deck) => {
                let (player_score, opponent_score) = partition_for(player, &scores);
                let scores = merge(player_score, opponent_score);
                let (player_hand, opponent_hand) = partition_for(player, &hands);
                let hands = merge(face_up(&player_hand.cards()), face_down(&opponent_hand.cards()));
                let crib = face_down(&crib.cards());
                let dealer = Dealer::from((dealer, player));
                Game::Discarding(scores, hands, crib, dealer)
            },
            DomainGame::Playing(scores, dealer, hands, play_state, cut, crib) => {
                let (player_score, opponent_score) = partition_for(player, &scores);
                let scores = merge(player_score, opponent_score);
                let (player_hand, opponent_hand) = partition_for(player, &hands);
                let hands = merge(face_up(&player_hand.cards()), face_down(&opponent_hand.cards()));
                let play_state = (play_state, player).into();
                let crib = face_down(&crib.cards());
                let dealer = Dealer::from((dealer, player));
                Game::Playing(scores, hands, play_state, cut, crib, dealer)
            },
            DomainGame::ScoringPone(scores, _, hands, cut, crib) => {
                let (player_score, opponent_score) = partition_for(player, &scores);
                let scores = merge(player_score, opponent_score);
                let (player_hand, opponent_hand) = partition_for(player, &hands);
                let hands = merge(face_up(&player_hand.cards()), face_down(&opponent_hand.cards()));
                let crib = face_down(&crib.cards());
                Game::ScoringPone(scores, hands, cut, crib)
            },
            DomainGame::Finished(_scores) => unimplemented!(),
        }
    }
}

fn partition_for<T: Clone>(player: DomainPlayer, map: &HashMap<DomainPlayer, T>) -> (T, T) {
    let (players, opponents): (HashMap<&DomainPlayer, &T>, HashMap<&DomainPlayer, &T>) =
        map.iter().partition(|(p, _)| **p == player);
    let players_t = players.into_values().next().unwrap();
    let opponents_t = opponents.into_values().take(1).next().unwrap();
    (players_t.clone(), opponents_t.clone())
}

fn merge<T>(players_t: T, opponents_t: T) -> HashMap<Role, T> {
    vec![
        (Role::CurrentPlayer, players_t),
        (Role::Opponent, opponents_t),
    ].into_iter().collect()
}

fn face_up(cards: &[Card]) -> Vec<CardSlot> {
    Vec::from_iter(cards.iter().map(|&c|CardSlot::FaceUp(c)))
}

fn face_down(cards: &[Card]) -> Vec<CardSlot> {
    Vec::from_iter(cards.iter().map(|_| CardSlot::FaceDown))
}
