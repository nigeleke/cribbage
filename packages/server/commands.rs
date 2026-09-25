use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::domain::{Card, GameId, Player, UserId};

/// Commands that drive the game state machine.
///
/// Each variant represents an action a player can take.
#[derive(Debug, Serialize, Deserialize, AsRefStr)]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type", content = "data")]
pub enum GameCommand {
    /// User wants to create a new game and become its host. The game will require a guest to join in order to continue.
    HostGame {
        /// User hosting the game.
        user_id: UserId,

        /// The new game identity.
        game_id: GameId,
    },

    /// User wants to join the game has a guest.
    JoinGame {
        /// User hosting the game.
        user_id: UserId,
    },

    /// Client wants to start a single-player game against the computer.
    PlayComputer {
        /// User creating the game.
        user_id: UserId,

        /// The new game identity.
        game_id: GameId,
    },

    /// Player cuts the deck to determine who deals first.
    CutForDeal {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    /// Player signals they are ready for the game to start.
    /// This is a `Pending` acknowledgment on the `CutForDeal` cuts.
    StartGame {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    /// Player discards two cards to the crib.
    DiscardCards {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The cards being discarded.
        cards: Vec<Card>,
    },

    /// Player plays a card during the `Playing` (or Pegging) phase.
    PlayCard {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The card to be played.
        card: Card,
    },

    /// Player declares go during the `Playing` (or Pegging) phase.
    Go {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    /// Player requests scoring of the non-dealer's hand.
    /// This is a `Pending` acknowledgment at the end of the `Playing` phase.
    ScorePone {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    /// Player requests scoring of the dealer's hand.
    /// This is a `Pending` acknowledgment at the end of the `ScoringPone` phase.
    ScoreDealer {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    /// Player requests scoring of the dealer's crib.
    /// This is a `Pending` acknowledgment at the end of the `ScoringDealer` phase.
    ScoreCrib {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    /// Player requests that the next `Discarding` round should start.
    /// This is a `Pending` acknowledgment at the end of the `ScoringCrib` phase.
    StartNextRound {
        /// The player identity (essentially host or guest)
        player: Player,
    },
}
