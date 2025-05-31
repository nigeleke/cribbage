use super::{active_game::ActiveGameId, unstarted_game::UnstartedGameId};
#[cfg(feature = "server")]
use crate::database::StartedGameRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartedGame {
    pub(crate) unstarted_game_id: UnstartedGameId,
    pub(crate) active_game_id: ActiveGameId,
}

impl StartedGame {
    pub fn unstarted_game_id(&self) -> &UnstartedGameId {
        &self.unstarted_game_id
    }

    pub fn active_game_id(&self) -> &ActiveGameId {
        &self.active_game_id
    }
}

#[cfg(feature = "server")]
impl From<StartedGameRow> for StartedGame {
    fn from(value: StartedGameRow) -> Self {
        Self {
            unstarted_game_id: UnstartedGameId::from(value.unstarted_game_id),
            active_game_id: ActiveGameId::from(value.active_game_id),
        }
    }
}
