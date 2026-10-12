use serde::{Deserialize, Serialize};
use tellus::{SchemaVersion, Versioned};

use crate::game::Users;
use crate::persistence::{PersistedCard, PersistedCards, PersistedDiscard};
use crate::user::UserId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    GameCreated {
        users: Users,
        deck: PersistedCards,
    },
    CutForDealMade {
        user: UserId,
    },
    HandsDealt {
        deck: PersistedCards,
    },
    DiscardMade {
        user: UserId,
        discard: PersistedDiscard,
    },
    StarterCut,
    CardPlayed {
        user: UserId,
        card: PersistedCard,
    },
    GoMade {
        user: UserId,
    },
    PoneScoreMade,
    DealerScoreMade,
    CribScoreMade,
}

impl Versioned for GameEvent {
    const MANIFEST: &'static str = "cribbage.game-event";
    const VERSION: SchemaVersion = SchemaVersion::new(1);
}
