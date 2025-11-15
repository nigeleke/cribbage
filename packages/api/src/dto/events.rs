use serde::{Deserialize, Serialize};

use crate::GameIdDTO;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEventDTO {
    LobbyGameCreated { name: String },
    OpponentJoined,
    // ComputerGameStarted {
    //     name: String,
    // },
    // CutForDealMade {
    //     player: PlayerDTO,
    //     cut: CardDTO,
    // },
    // CutForDealAcknowledged {
    //     player: PlayerDTO,
    // },
    CutForDealTied,
    CutForDealDecided,
    // HandDealt {
    //     player: PlayerDTO,
    //     hand: Vec<CardDTO>,
    // },
    // CardsDiscardedToCrib {
    //     player: PlayerDTO,
    // },
    // StarterSelected {
    //     cut: CardDTO,
    // },
    // // PointsScored {
    // //     points: Points,
    // //     reasons: ScoreBreakdown,
    // // },
    // CardPlayed {
    //     player: PlayerDTO,
    //     card: CardDTO,
    // },
    // Passed {
    //     player: PlayerDTO,
    // },
    // PlayCompleted,
    // PoneHandRevealed {
    //     player: PlayerDTO,
    //     hand: Vec<CardDTO>,
    // },
    // DealerHandRevealed {
    //     player: PlayerDTO,
    //     hand: Vec<CardDTO>,
    // },
    // CribRevealed {
    //     player: PlayerDTO,
    //     crib: Vec<CardDTO>,
    // },
    // WinnerDeclared {
    //     player: PlayerDTO,
    // },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailableGameEventDTO {
    Created { game_id: GameIdDTO, name: String },
    Removed { game_id: GameIdDTO, name: String },
}
