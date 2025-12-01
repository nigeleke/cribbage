use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[cfg(test)]
use crate::domain::Game;
use crate::domain::{Card, Dealer, GameId, Hand, Player, ScoreBreakdown, StarterCut, UserId};

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
    ComputerGameCreated {
        game_id: GameId,
        host: UserId,
        guest: UserId,
        name: String,
    },
    CutForDealMade {
        player: Player,
        cut: Card,
    },
    GameStarted {
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
    CardsDiscarded {
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
    PoneScored {
        player: Player,
    },
    DealerScored {
        player: Player,
    },
    CribScored {
        player: Player,
    },
    NextRoundStarted {
        player: Player,
    },
    WinnerDeclared {
        player: Player,
    },

    #[cfg(test)]
    GamePreloaded {
        game: Game,
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
