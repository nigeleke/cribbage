use crate::{
    display::format_hashmap,
    domain::{
        Crib, Cut, Hands, HasCrib, HasCut, HasHands, HasPlayers, HasScores, Players, Roles, Scores,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finished {
    pub scores: Scores,
    pub roles: Roles,
    pub hands: Hands,
    pub crib: Crib,
    pub cut: Cut,
}

impl HasPlayers for Finished {
    fn players(&self) -> Players {
        Players::from_iter(self.hands.keys().copied())
    }
}

impl HasScores for Finished {
    fn scores(&self) -> &Scores {
        &self.scores
    }
}

impl HasHands for Finished {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasCrib for Finished {
    fn crib(&self) -> &Crib {
        &self.crib
    }
}

impl HasCut for Finished {
    fn cut(&self) -> Cut {
        self.cut
    }
}

impl std::fmt::Display for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Finished(
    scores: {},
    roles: {},
    hands: {},
    crib: {},
    cut: {}
)"#,
            self.scores,
            self.roles,
            format_hashmap(&self.hands),
            self.crib,
            self.cut
        )
    }
}
