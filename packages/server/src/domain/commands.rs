use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::domain::{Card, GameId, Player, UserId};

#[derive(Debug, Serialize, Deserialize, AsRefStr)]
pub enum GameCommand {
    HostGame { user_id: UserId, game_id: GameId },
    JoinGame { user_id: UserId },
    PlayComputer { user_id: UserId, game_id: GameId },
    CutForDeal { player: Player },
    StartGame { player: Player },
    DiscardCards { player: Player, cards: Vec<Card> },
    PlayCard { player: Player, card: Card },
    Go { player: Player },
    ScorePone { player: Player },
    ScoreDealer { player: Player },
    ScoreCrib { player: Player },
    StartNextRound { player: Player },
}
