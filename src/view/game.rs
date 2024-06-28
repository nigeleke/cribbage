use super::card::{Card, CardSlot, Cut};
use super::cards::{Crib, Cuts, Hands};
use super::plays::PlayState;
use super::role::{Dealer, Role};
use super::scores::{Peggings, Scores};

use crate::domain::Game as DomainGame;
use crate::types::Player;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Game {
    Starting(Cuts),
    Discarding(Scores, Hands, Crib, Dealer),
    Playing(Scores, Hands, PlayState, Cut, Crib, Dealer),
    ScoringPone(Scores, Role, Hands, Cut, Crib),
    ScoringDealer(Scores, Role, Hands, Cut, Crib),
    ScoringCrib(Scores, Role, Hands, Cut, Crib),
    Finished(Role, Peggings),
}

impl From<(DomainGame, Player)> for Game {
    fn from((game, player): (DomainGame, Player)) -> Self {
        match game {
            DomainGame::Starting(cuts, _) => {
                let (player_cut, opponent_cut) = partition_for(player, &cuts);
                let cuts = merge(player_cut, opponent_cut);
                Game::Starting(cuts)
            },
            DomainGame::Discarding(scores, dealer, hands, crib, _deck) => {
                let (player_pegging, opponent_pegging) = partition_for(player, &scores.peggings());
                let peggings = merge(player_pegging, opponent_pegging);
                let scores = Scores::new(peggings, scores.reasons());
                let (player_hand, opponent_hand) = partition_for(player, &hands);
                let hands = merge(face_up(player_hand.as_ref()), face_down(opponent_hand.as_ref()));
                let crib = face_down(crib.as_ref());
                let dealer = Dealer::from((dealer, player));
                Game::Discarding(scores, hands, crib, dealer)
            },
            DomainGame::Playing(scores, dealer, hands, play_state, cut, crib) => {
                let (player_pegging, opponent_pegging) = partition_for(player, &scores.peggings());
                let peggings = merge(player_pegging, opponent_pegging);
                let scores = Scores::new(peggings, scores.reasons());
                let (player_hand, opponent_hand) = partition_for(player, &hands);
                let hands = merge(face_up(player_hand.as_ref()), face_down(opponent_hand.as_ref()));
                let play_state = (play_state, player).into();
                let crib = face_down(crib.as_ref());
                let dealer = Dealer::from((dealer, player));
                Game::Playing(scores, hands, play_state, cut, crib, dealer)
            },
            DomainGame::ScoringPone(ref scores, _, ref hands, cut, ref crib) => {
                let pone = Role::from((game.pone(), player));
                let (player_pegging, opponent_pegging) = partition_for(player, &scores.peggings());
                let peggings = merge(player_pegging, opponent_pegging);
                let scores = Scores::new(peggings, scores.reasons());
                let (player_hand, opponent_hand) = partition_for(player, hands);
                let hands = merge(face_up(player_hand.as_ref()), face_up(opponent_hand.as_ref()));
                let crib = face_down(crib.as_ref());
                Game::ScoringPone(scores, pone, hands, cut, crib)
            },
            DomainGame::ScoringDealer(ref scores, _, ref hands, cut, ref crib) => {
                let pone = Role::from((game.pone(), player));
                let (player_pegging, opponent_pegging) = partition_for(player, &scores.peggings());
                let peggings = merge(player_pegging, opponent_pegging);
                let scores = Scores::new(peggings, scores.reasons());
                let (player_hand, opponent_hand) = partition_for(player, hands);
                let hands = merge(face_up(player_hand.as_ref()), face_up(opponent_hand.as_ref()));
                let crib = face_down(crib.as_ref());
                Game::ScoringDealer(scores, pone, hands, cut, crib)
            },
            DomainGame::ScoringCrib(ref scores, _, ref hands, cut, ref crib) => {
                let pone = Role::from((game.pone(), player));
                let (player_pegging, opponent_pegging) = partition_for(player, &scores.peggings());
                let peggings = merge(player_pegging, opponent_pegging);
                let scores = Scores::new(peggings, scores.reasons());
                let (player_hand, opponent_hand) = partition_for(player, hands);
                let hands = merge(face_up(player_hand.as_ref()), face_up(opponent_hand.as_ref()));
                let crib = face_down(crib.as_ref());
                Game::ScoringCrib(scores, pone, hands, cut, crib)
            },
            DomainGame::Finished(winner, ref peggings) => {
                let winner = Role::from((winner, player));
                let (player_pegging, opponent_pegging) = partition_for(player, &peggings);
                let peggings = merge(player_pegging, opponent_pegging);
                Game::Finished(winner, peggings)
            },
        }
    }
}

fn partition_for<T: Clone>(player: Player, map: &HashMap<Player, T>) -> (T, T) {
    let (players, opponents): (HashMap<&Player, &T>, HashMap<&Player, &T>) =
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
