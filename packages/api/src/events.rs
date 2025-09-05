use crate::{Card, Cut, Dealer, GameId, Player, ScoreBreakdown, Scoreboard, UserId, Users};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

/// Domain events occuring as a result of user actions.
#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum Event {
    /// A new game has been created in the lobby.
    LobbyGameCreated {
        /// The unique ID of the game.
        game_id: GameId,
        /// The user who created the game.
        host: UserId,
        /// The name given to the game.
        name: String
    },

    /// A guest has joined an existing lobby game.
    LobbyGameJoined {
        /// The ID of the game being joined.
        game_id: GameId,
        /// The user who joined as a guest.
        guest: UserId
    },

    /// The game has started.
    ComputerGameStarted {
        /// The ID of the started game.
        game_id: GameId,
        /// All users participating in the game.
        users: Users,
        /// The name of the game.
        name: String
    },

    /// A player has cut a card to determine the dealer.
    CardCutForDeal {
        /// The game in which the cut occurred.
        game_id: GameId,
        /// The player who made the cut.
        player: Player,
        /// The card that was cut.
        cut: Cut,
    },

    /// TODO:
    RedrawRequested {
        game_id: GameId,
    },

    /// TODO:
    RoundStarted {
        game_id: GameId,
        dealer: Dealer,
        scoreboard: Scoreboard,
    },

    CardsDiscardedToCrib {
        game_id: GameId,
        player: Player,
        discards: Vec<Card>,
    },

    CardCutAtStartOfPlay {
        game_id: GameId,
        cut: Cut,
    },

    ScoreRecorded {
        game_id: GameId,
        player: Player,
        breakdown: ScoreBreakdown
    },

    WinnerDeclared {
        game_id: GameId,
        winner: Player,
    },

    CardPlayed {
        game_id: GameId,
        player: Player,
        card: Card,
    },

    Passed {
        game_id: GameId,
        player: Player,
    }

}
