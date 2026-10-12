use crate::game::Users;
use crate::persistence::{PersistedCard, PersistedCards, PersistedDiscard};
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
        discard: PersistedDiscard,
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
