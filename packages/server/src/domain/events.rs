use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[cfg(test)]
use crate::domain::Game;
use crate::domain::{Card, Dealer, GameId, Hand, Pegging, Player, StarterCut, UserId};

/// Domain events which represent the **single source of truth** for game history and
/// are persisted in the event store. The current game state is derived by folding them
/// in order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum GameEvent {
    /// A user created a lobby game and is waiting for another user to join
    LobbyGameCreated {
        /// The identity of the game that was created.
        game_id: GameId,

        /// The user that created the game.
        host: UserId,

        /// The name assigned to the game. It's possible this may be
        /// the same as another game; the `game_id` provides the unique id.
        name: String,
    },

    /// A user joined a game that was previously a lobby game.
    LobbyGameJoined {
        /// The user that joined the game.
        guest: UserId,
    },

    /// A user a computer game.
    ComputerGameCreated {
        /// The identity of the game that was created.
        game_id: GameId,

        /// The user that created the game.
        host: UserId,

        /// The computer "user" that will play against the host.
        guest: UserId,

        /// The name assigned to the game. It's possible this may be
        /// the same as another game; the `game_id` provides the unique id.
        name: String,
    },

    /// A player cut a card for the choice of initial dealer.
    CutForDealMade {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The card that was cut.
        cut: Card,
    },

    /// A player has acknowledged the cut for deal. When both players
    /// have acknowledge the cards will be dealt and discarding begin.
    GameStarted {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    /// Both plays have cut for deal (and acknowledged the cut); the
    /// cuts were the same rank, so the cut needs to be redrawn.
    CutForDealTied,

    /// Both plays have cut for deal (and acknowledged the cut); the
    /// cuts were different, so the dealer is selected and the deal
    /// can progress.
    CutForDealDecided {
        /// The selected dealer.
        dealer: Dealer,
    },

    /// A hand has been dealt to the player, following `CutForDealDecided`
    /// or `NextRoundStarted`.
    HandDealt {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The hand that has been dealt.
        hand: Hand,
    },

    /// A player has discarded cards to the crib.
    CardsDiscarded {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The cards that were discarded.
        cards: Vec<Card>,
    },

    /// Both players have discarded cards to the crib and the "Starter"
    /// card selected.
    StarterSelected {
        /// The starter card.
        cut: StarterCut,

        /// Record `HisHeels`.
        pegging: Pegging,
    },

    /// A player has played a card during the `Playing` (or Pegging) phase.
    CardPlayed {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The card that was played.
        card: Card,

        /// Points awarded during pegging (may include 31 for 2).
        pegging: Pegging,
    },

    /// A player has called `Go` during the `Playing` (or Pegging) phase.
    GoCalled {
        /// The player identity (essentially host or guest)
        player: Player,

        /// Points awarded for the `Go`.
        pegging: Pegging,
    },

    /// A player has `acknowledged` `End of Plays`; when both players
    /// have acknowledged then the Pone's hand will be scored.
    PoneScored {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The pone's hand scores.
        pegging: Pegging,
    },

    /// A player has `acknowledged` the `Pone Score`; when both players
    /// have acknowledged then the Dealer's hand will be scored.
    DealerScored {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The dealer's hand scores.
        pegging: Pegging,
    },

    /// A player has `acknowledged` the `Dealer Score`; when both players
    /// have acknowledged then the Dealer's crib will be scored.
    CribScored {
        /// The player identity (essentially host or guest)
        player: Player,

        /// The dealer's crib scores.
        pegging: Pegging,
    },

    /// A player has `acknowledged` the `Crib Score`; when both players
    /// have acknowledged then the next round will start, swapping dealers.
    NextRoundStarted {
        /// The player identity (essentially host or guest)
        player: Player,
    },

    #[cfg(test)]
    /// A preset game state has been loaded, to enable easy test setup.
    /// This event is emitted during testing only; it may not represent
    /// a 'real' state, but rather a suitable state to enable the test.
    /// This is boxed to avoid large variant size difference clippy warnings.
    GamePreloaded {
        /// The full preloaded game object.
        game: Box<Game>,
    },
}

impl DomainEvent for GameEvent {
    fn event_type(&self) -> String {
        String::from(self.as_ref())
    }

    fn event_version(&self) -> String {
        String::from(env!("CARGO_PKG_VERSION"))
    }
}
