use cribbage_domain::constants::CARDS_DISCARDED_TO_CRIB;

use crate::game::Users;
use crate::persistence::{PersistedCard, PersistedCards};
use crate::user::UserId;

#[derive(Debug)]
pub enum GameCommand {
    CreateGame {
        users: Users,
        deck: PersistedCards,
    },
    CutForDeal {
        user: UserId,
    },
    DealHands {
        deck: PersistedCards,
    },
    Discard {
        user: UserId,
        discard: [PersistedCard; CARDS_DISCARDED_TO_CRIB],
    },
    CutStarter,
    PlayCard {
        user: UserId,
        card: PersistedCard,
    },
    Go {
        user: UserId,
    },
    ScorePone,
    ScoreDealer,
    ScoreCrib,
}
