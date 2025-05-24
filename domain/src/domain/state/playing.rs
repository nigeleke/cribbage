use serde::{Deserialize, Serialize};

use crate::{
    display::format_hashmap,
    domain::{
        Crib, Cut, Hands, HasCrib, HasCut, HasHands, HasPlayers, HasRoles, HasScores, PlayState,
        Players, Roles, Scores,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Playing {
    pub scores: Scores,
    pub roles: Roles,
    pub hands: Hands,
    pub play_state: PlayState,
    pub crib: Crib,
    pub cut: Cut,
}

impl HasPlayers for Playing {
    fn players(&self) -> Players {
        Players::from_iter(self.hands.keys().copied())
    }
}

impl HasScores for Playing {
    fn scores(&self) -> &Scores {
        &self.scores
    }
}

impl HasRoles for Playing {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasHands for Playing {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasCrib for Playing {
    fn crib(&self) -> &Crib {
        &self.crib
    }
}

impl HasCut for Playing {
    fn cut(&self) -> Cut {
        self.cut
    }
}

impl std::fmt::Display for Playing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Playing(
    scores: {},
    roles: {},
    hands: {},
    play_state: {},
    cut: {},
    crib: {}
)"#,
            self.scores,
            self.roles,
            format_hashmap(&self.hands),
            self.play_state,
            self.cut,
            self.crib
        )
    }
}
