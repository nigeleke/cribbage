use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::domain::{Card, GameId, Player, UserId};

#[derive(Debug, Serialize, Deserialize, AsRefStr)]
pub enum GameCommand {
    HostGame { user_id: UserId, game_id: GameId },
    JoinGame { user_id: UserId },
    PlayComputer { user_id: UserId, game_id: GameId },
    CutForDeal { player: Player },
    AcknowledgeCutForDeal { player: Player },
    DiscardCardsToCrib { player: Player, cards: Vec<Card> },
    PlayCard { player: Player, card: Card },
    Pass { player: Player },
    AcknowledgePlaysEnded { player: Player },
    AcknowledgePoneScore { player: Player },
    AcknowledgeDealerScore { player: Player },
    AcknowledgeCribScore { player: Player },
}
