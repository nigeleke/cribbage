use crate::{
    display::format_hashmap,
    domain::{
        Crib, Cut, Hands, HasCrib, HasCut, HasHands, HasPlayers, HasScores, Player, Players, Roles,
        Scores,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finished {
    winner: Player,
    scores: Scores,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    cut: Cut,
}

impl Finished {
    #[rustfmt::skip]
    pub fn new(winner: Player, scores: Scores, roles: Roles, hands: Hands, crib: Crib, cut: Cut) -> Self {
        Self { winner, scores, roles, hands, crib, cut }
    }

    pub fn winner(&self) -> Player {
        self.winner
    }
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
        #[rustfmt::skip]
        let Self { winner, scores, roles, hands, crib, cut } = self;
        let hands = format_hashmap(hands);

        write!(
            f,
            r#"Finished(
    winner: {winner},
    scores: {scores},
    roles: {roles},
    hands: {hands},
    crib: {crib},
    cut: {cut}
)"#
        )
    }
}
