use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::domain::{
    Card, Crib, Dealer, GameId, Hand, Player, Points, ScoreBreakdown, StarterCut, UserId,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum GameEvent {
    LobbyGameCreated {
        game_id: GameId,
        host: UserId,
        name: String,
    },
    LobbyGameJoined {
        guest: UserId,
    },
    ComputerGameStarted {
        game_id: GameId,
        host: UserId,
        guest: UserId,
        name: String,
    },
    CutForDealMade {
        player: Player,
        cut: Card,
    },
    CutForDealAcknowledged {
        player: Player,
    },
    CutForDealTied,
    CutForDealDecided {
        dealer: Dealer,
    },
    HandDealt {
        player: Player,
        hand: Hand,
    },
    CardsDiscardedToCrib {
        player: Player,
        cards: Vec<Card>,
    },
    StarterSelected {
        cut: StarterCut,
    },
    PointsScored {
        player: Player,
        reasons: ScoreBreakdown,
    },
    CardPlayed {
        player: Player,
        card: Card,
    },
    Passed {
        player: Player,
    },
    PlayCompleted,
    PoneHandRevealed {
        hand: Hand,
    },
    DealerHandRevealed {
        hand: Hand,
    },
    CribRevealed {
        crib: Crib,
    },
    WinnerDeclared {
        player: Player,
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
