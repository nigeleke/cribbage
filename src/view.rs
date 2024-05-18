use crate::domain::prelude::{Game as DomainGame, Player as DomainPlayer};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type Card = crate::domain::prelude::Card;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OpponentCut(Card);

impl From<Card> for OpponentCut {
    fn from(value: Card) -> Self {
        OpponentCut(value)
    }
}

impl From<OpponentCut> for Card {
    fn from(value: OpponentCut) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PlayerCut(Card);

impl From<Card> for PlayerCut {
    fn from(value: Card) -> Self {
        PlayerCut(value)
    }
}

impl From<PlayerCut> for Card {
    fn from(value: PlayerCut) -> Self {
        value.0
    }
}

// /// The view of an opponent's card.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum OpponentCard {
//     Empty,
//     FaceDown,
//     FaceUp(Card),
// }

// impl OpponentCard {
//     pub fn rank(&self) -> usize {
//         match self {
//             OpponentCard::Empty => 0,
//             OpponentCard::FaceDown => 0,
//             OpponentCard::FaceUp(card) => card.rank(),
//         }
//     }
// }

// /// The view of a player's card.
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub enum PlayerCard {
//     Empty,
//     FaceUp(Card),
// }

// impl PlayerCard {
//     pub fn rank(&self) -> usize {
//         match self {
//             PlayerCard::Empty => 0,
//             PlayerCard::FaceUp(card) => card.rank(),
//         }
//     }
// }

// /// A collection of cards to show.
// #[derive(Serialize, Deserialize, Debug)]
// struct Cards(Vec<CardView>);

// /// A player's hand.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Hand(Cards);

// /// The current crib.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Crib(Cards);

// /// The current cut.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Cut(CardView);

// /// A points value.
// #[derive(Serialize, Deserialize, Debug)]
// struct Points(i32);

// /// A score comprises the back and front pegs.
// #[derive(Serialize, Deserialize, Debug)]
// struct Score {
//     back_peg: Points,
//     front_peg: Points,
// }

// /// My score.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct MyScore(Score);

// /// The opponent's score.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct OpponentScore(Score);

// #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
// pub struct PlayerId(Ulid);

// impl PlayerId {
//     pub fn new() -> Self { PlayerId(Ulid::new()) }
// }

// impl fmt::Display for PlayerId {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// /// A player is either the current session or not.
// #[derive(Serialize, Deserialize, Debug)]
// pub enum Player {
//     Me,
//     Opponent,
// }

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
    Starting(PlayerCut, OpponentCut),
    Discarding(String),
// //     Playing(MyState, OpponentState, Play, Cut, Crib),
// //     ScoringPoneCards(MyState, OpponentState, Cut, Crib),
// //     ScoringDealerCards(MyState, OpponentState, Cut, Crib),
// //     ScoringCrib(MyState, OpponentState, Cut, Crib),
// //     Finished(MyScore, OpponentScore),
}

impl From<(DomainGame, DomainPlayer)> for Game {
    fn from((game, player): (DomainGame, DomainPlayer)) -> Self {
        match game {
            DomainGame::Starting(cuts, _) => {
                let (player_cut, opponent_cut) = partition_for(player, &cuts);
                Game::Starting(player_cut.into(), opponent_cut.into())
            },
            DomainGame::Discarding(scores, dealer, hands, crib, deck) => {
                let (player_score, opponent_score) = partition_for(player, &scores);
                let (player_hand, opponent_hand) = partition_for(player, &hands);
                let state = format!("Scores({} / {}), Hands({} / {}), Dealer({}), {}, {}", player_score, opponent_score, player_hand, opponent_hand, dealer, crib, deck);
                Game::Discarding(state)
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
