use crate::display::format_hashmap;
use crate::domain::{
    Crib, Cut, Hands, HasCrib, HasCut, HasHands, HasPlayState, HasPlayers, HasRoles, HasScores,
    PlayState, Players, Roles, Scores,
};

#[derive(Debug)]
pub struct Playing {
    scores: Scores,
    roles: Roles,
    hands: Hands,
    play_state: PlayState,
    cut: Cut,
    crib: Crib,
}

impl Playing {
    pub fn new(
        scores: Scores,
        roles: Roles,
        hands: Hands,
        play_state: PlayState,
        cut: Cut,
        crib: Crib,
    ) -> Self {
        Self {
            scores,
            roles,
            hands,
            play_state,
            cut,
            crib,
        }
    }

    pub fn into_parts(self) -> (Scores, Roles, Hands, PlayState, Cut, Crib) {
        let Self {
            scores,
            roles,
            hands,
            play_state,
            cut,
            crib,
        } = self;
        (scores, roles, hands, play_state, cut, crib)
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

impl HasPlayState for Playing {
    fn play_state(&self) -> &PlayState {
        &self.play_state
    }
}

impl HasCut for Playing {
    fn cut(&self) -> Cut {
        self.cut
    }
}

impl HasCrib for Playing {
    fn crib(&self) -> &Crib {
        &self.crib
    }
}

impl std::fmt::Display for Playing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Playing(scores: {}, roles: {}, hands: {}, play_state: {}, cut: {}, crib: {})",
            self.scores,
            self.roles,
            format_hashmap(&self.hands),
            self.play_state,
            self.cut,
            self.crib
        )
    }
}
