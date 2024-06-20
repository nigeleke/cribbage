use super::card::{Card, CardSlot, Cut};
use super::cards::{Crib, Cuts, Hands};
use super::plays::PlayState;
use super::role::{Dealer, Role};
use super::pegging::Peggings;

use crate::domain::prelude::Game as DomainGame;
use crate::types::prelude::Player;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Game {
    Starting(Cuts),
    Discarding(Peggings, Hands, Crib, Dealer),
    Playing(Peggings, Hands, PlayState, Cut, Crib, Dealer),
    ScoringPone(Peggings, Role, Hands, Cut, Crib),
    ScoringDealer(Peggings, Role, Hands, Cut, Crib),
    ScoringCrib(Peggings, Role, Hands, Cut, Crib),
    Finished(Peggings),
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
                // TODO: Change opponent face_up back to face_down.
                let hands = merge(face_up(&player_hand.cards()), face_up(&opponent_hand.cards()));
                let play_state = (play_state, player).into();
                let crib = face_down(&crib.cards());
                let dealer = Dealer::from((dealer, player));
                Game::Playing(scores, hands, play_state, cut, crib, dealer)
            },
            DomainGame::ScoringPone(ref scores, _, ref hands, cut, ref crib) => {
                let pone = Role::from((game.pone(), player));
                let (player_score, opponent_score) = partition_for(player, scores);
                let scores = merge(player_score, opponent_score);
                let (player_hand, opponent_hand) = partition_for(player, hands);
                let hands = merge(face_up(&player_hand.cards()), face_up(&opponent_hand.cards()));
                let crib = face_down(&crib.cards());
                Game::ScoringPone(scores, pone, hands, cut, crib)
            },
            DomainGame::ScoringDealer(ref scores, _, ref hands, cut, ref crib) => {
                let pone = Role::from((game.pone(), player));
                let (player_score, opponent_score) = partition_for(player, scores);
                let scores = merge(player_score, opponent_score);
                let (player_hand, opponent_hand) = partition_for(player, hands);
                let hands = merge(face_up(&player_hand.cards()), face_up(&opponent_hand.cards()));
                let crib = face_down(&crib.cards());
                Game::ScoringDealer(scores, pone, hands, cut, crib)
            },
            DomainGame::ScoringCrib(ref scores, _, ref hands, cut, ref crib) => {
                let pone = Role::from((game.pone(), player));
                let (player_score, opponent_score) = partition_for(player, scores);
                let scores = merge(player_score, opponent_score);
                let (player_hand, opponent_hand) = partition_for(player, hands);
                let hands = merge(face_up(&player_hand.cards()), face_up(&opponent_hand.cards()));
                let crib = face_down(&crib.cards());
                Game::ScoringCrib(scores, pone, hands, cut, crib)
            },
            DomainGame::Finished(ref scores) => {
                let (player_score, opponent_score) = partition_for(player, scores);
                let scores = merge(player_score, opponent_score);
                Game::Finished(scores)
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
