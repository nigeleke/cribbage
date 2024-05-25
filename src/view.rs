use crate::domain::prelude::{
    Game as DomainGame,
    Player as DomainPlayer
};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Role {
    CurrentPlayer,
    Opponent
} 

type Dealer = Role;

impl From<(DomainPlayer, DomainPlayer)> for Dealer {
    fn from((current, dealer): (DomainPlayer, DomainPlayer)) -> Self {
        if current == dealer {
            Role::CurrentPlayer
        } else {
            Role::Opponent
        }
    }
}

pub type Card = crate::domain::prelude::Card;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CardSlot {
    FaceUp(Card),
    FaceDown,
    Empty,
    Placeholder,
}

pub type Cuts = HashMap<Role, Card>;

pub type Cut = Card;

pub type Hand = Vec<CardSlot>;

pub type Hands = HashMap<Role, Hand>;

pub type Crib = Vec<CardSlot>;

pub type Score = crate::domain::prelude::Score;

pub type Scores = HashMap<Role, Score>;

// /// A player state is their score, and the cards currently held in their hand.
// #[derive(Serialize, Deserialize, Debug)]
// struct PlayerState {
//     score: Score,
//     hand: Cards,
// }

// /// My current hand & score.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct MyState(PlayerState);

// /// Opponent's current hand & score.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct OpponentState(PlayerState);

// /// During play, a player lays a card. Passes are not shown.
// #[derive(Serialize, Deserialize, Debug)]
// struct Lay {
//     player: Player,
//     card: Card,
// }

// /// A collection of lays.
// #[derive(Serialize, Deserialize, Debug)]
// struct Lays(Vec<Lay>);

// /// The play state is the next player to play, the current cards laid and historic cards laid.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Play {
//     next_player: Player,
//     current_play: Lays,
//     historic_plays: Lays,
// }

// #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
// pub struct GameId(Ulid);

// impl GameId {
//     pub fn new() -> Self { GameId(Ulid::new()) }
// }

// impl fmt::Display for GameId {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

/// The game state, waiting for opponent, discarding, playing, scoring, finished.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Game {
    Starting(Cuts),
    Discarding(Scores, Hands, Crib, Dealer),
    Playing(Scores, Hands, Cut, Crib, Dealer),
// //     ScoringPoneCards(MyState, OpponentState, Cut, Crib),
// //     ScoringDealerCards(MyState, OpponentState, Cut, Crib),
// //     ScoringCrib(MyState, OpponentState, Cut, Crib),
// //     Finished(MyScore, OpponentScore),
}

impl Game {
    pub(crate) fn scores(&self) -> Scores {
        match self {
            Game::Starting(_) => Scores::new(),
            Game::Discarding(scores, _, _, _) => scores.clone(),
            Game::Playing(scores, _, _, _, _) => scores.clone(),
        }
    }

    pub(crate) fn dealer(&self) -> Option<Role> {
        match self {
            Game::Starting(_) => None,
            Game::Discarding(_, _, _, dealer) => Some(dealer.clone()),
            Game::Playing(_, _, _, _, dealer) => Some(dealer.clone()),
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
                let crib = face_down(&crib.cards());
                let dealer = Dealer::from((dealer, player));
                Game::Playing(scores, hands, cut, crib, dealer)
            },
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