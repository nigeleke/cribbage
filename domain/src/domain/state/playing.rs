use crate::{
    display::format_hashmap,
    domain::{
        Crib, Cut, Hands, HasCrib, HasCut, HasHands, HasPlayers, HasRoles, HasScores, PlayState,
        Players, Roles, Scores,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playing {
    scores: Scores,
    roles: Roles,
    hands: Hands,
    play_state: PlayState,
    crib: Crib,
    cut: Cut,
}

impl Playing {
    #[rustfmt::skip]
    pub fn new(scores: Scores, roles: Roles, hands: Hands, play_state: PlayState, crib: Crib, cut: Cut) -> Self {
        Self { scores, roles, hands, play_state, crib, cut }
    }

    pub fn into_parts(self) -> (Scores, Roles, Hands, PlayState, Crib, Cut) {
        #[rustfmt::skip]
        let Self { scores, roles, hands, play_state, crib, cut } = self;
        (scores, roles, hands, play_state, crib, cut)
    }

    pub const fn play_state(&self) -> &PlayState {
        &self.play_state
    }
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
        #[rustfmt::skip]
        let Self { scores, roles, hands, play_state, cut, crib } = self;
        let hands = format_hashmap(hands);

        write!(
            f,
            r#"Playing(
    scores: {scores},
    roles: {roles},
    hands: {hands},
    play_state: {play_state},
    cut: {cut},
    crib: {crib}
)"#
        )
    }
}
