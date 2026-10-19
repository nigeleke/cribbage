use serde::{Deserialize, Serialize};
use tellus::{SchemaVersion, Versioned};

use crate::game::{PersistedCard, PersistedCards, PersistedDiscard, Users};
use crate::user::UserId;

/// An event recording a change in the game.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    /// Records the creation of a game.
    GameCreated {
        /// The users participating in the game.
        users: Users,
        /// The deck assigned for the cuts for deal.
        deck: PersistedCards,
    },

    /// Records that a player cut for the deal.
    CutForDealMade {
        /// The user who cut the deck.
        user: UserId,
    },

    /// Records the dealing of the players' hands.
    HandsDealt {
        /// The deck used to deal the hands.
        deck: PersistedCards,
    },

    /// Records the cards discarded by a player.
    DiscardMade {
        /// The user who discarded the cards.
        user: UserId,
        /// The two cards discarded by the user.
        discard: PersistedDiscard,
    },

    /// Records the cutting of the starter card.
    StarterCut,

    /// Records a card played by a player.
    CardPlayed {
        /// The user who played the card.
        user: UserId,
        /// The card that was played.
        card: PersistedCard,
    },

    /// Records a player's declaration of go.
    GoMade {
        /// The user who declared go.
        user: UserId,
    },

    /// Records the scoring of the pone's hand.
    PoneScoreMade,

    /// Records the scoring of the dealer's hand.
    DealerScoreMade,

    /// Records the scoring of the crib.
    CribScoreMade,
}

impl Versioned for GameEvent {
    /// The identifier of the game-event schema.
    const MANIFEST: &'static str = "cribbage.game-event";
    /// The version of the game-event schema.
    const VERSION: SchemaVersion = SchemaVersion::new(1);
}
