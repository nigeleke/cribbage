use serde::{Deserialize, Serialize};

use crate::domain::{GameId, State, UserId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    game_id: GameId,
    kind: EventKind,
}

impl Event {
    pub fn new(game_id: GameId, kind: EventKind) -> Self {
        Self { game_id, kind }
    }

    pub fn lobby_game_created(id: GameId, host: UserId, name: String) -> Self {
        Self::new(id, EventKind::LobbyGameCreated { id, host, name })
    }

    pub fn computer_game_created(id: GameId, host: UserId, guest: UserId, name: String) -> Self {
        Self::new(
            id,
            EventKind::ComputerGameCreated {
                id,
                host,
                guest,
                name,
            },
        )
    }

    pub fn lobby_game_joined(id: GameId, guest: UserId) -> Self {
        Self::new(id, EventKind::LobbyGameJoined { id, guest })
    }

    pub fn state_updated(id: GameId, state: State) -> Self {
        Self::new(id, EventKind::StateUpdated { id, state })
    }

    pub fn id(&self) -> &GameId {
        &self.game_id
    }

    pub fn kind(&self) -> &EventKind {
        &self.kind
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    LobbyGameCreated {
        id: GameId,
        host: UserId,
        name: String,
    },
    ComputerGameCreated {
        id: GameId,
        host: UserId,
        guest: UserId,
        name: String,
    },
    LobbyGameJoined {
        id: GameId,
        guest: UserId,
    },
    StateUpdated {
        id: GameId,
        state: State,
    },
}
