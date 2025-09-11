use crate::{
    Card, Cut, Dealer, GameId, Player, ScoreBreakdown, ScorePhase, Scoreboard, UserId, Users,
};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    game_id: GameId,
    kind: EventKind,
}

impl Event {
    pub fn new(game_id: GameId, kind: EventKind) -> Self {
        Self { game_id, kind }
    }

    pub fn id(&self) -> &GameId {
        &self.game_id
    }

    pub fn kind(&self) -> &EventKind {
        &self.kind
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum EventKind {
    LobbyGameCreated {
        host: UserId,
        name: String,
    },

    LobbyGameJoined {
        guest: UserId,
    },

    ComputerGameStarted {
        users: Users,
        name: String,
    },

    CardCutForDeal {
        player: Player,
        cut: Cut,
    },

    RedrawRequested,

    RoundStarted {
        dealer: Dealer,
        scoreboard: Scoreboard,
    },

    CardsDiscardedToCrib {
        player: Player,
        discards: Vec<Card>,
    },

    StarterCardCut {
        cut: Cut,
    },

    ScoreRecorded {
        player: Player,
        phase: ScorePhase,
        breakdown: ScoreBreakdown,
    },

    CardPlayed {
        player: Player,
        card: Card,
    },

    Passed {
        player: Player,
    },

    PlaysFinished,

    PoneHandScored {
        breakdown: ScoreBreakdown,
    },

    PoneHandScoreAcknowledged {
        player: Player,
    },

    DealerHandScored {
        breakdown: ScoreBreakdown,
    },

    DealerHandScoreAcknowledged {
        player: Player,
    },
    CribScored {
        breakdown: ScoreBreakdown,
    },
    CribScoreAcknowledged {
        player: Player,
    },
    WinnerDeclared {
        winner: Player,
    },
}
