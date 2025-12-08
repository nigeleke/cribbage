use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{
        Card, Crib, Hands, HasCrib, HasHands, HasPending, HasPlayState, HasRoles, HasScoreboard,
        HasStarterCut, Pending, PlayState, Roles, Scoreboard, StarterCut,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Playing {
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    play_state: PlayState,
    crib: Crib,
    starter_cut: StarterCut,
    pending: Pending,
}

impl Playing {
    #[rustfmt::skip]
    pub const fn new(scoreboard: Scoreboard, roles: Roles, hands: Hands, play_state: PlayState, crib: Crib, starter_cut: StarterCut, pending: Pending) -> Self {
        Self { scoreboard, roles, hands, play_state, crib, starter_cut, pending }
    }

    pub fn play_card(&mut self, card: Card) {
        let player = self.play_state.next_to_play();

        let hand = &mut self.hands[player];
        hand.remove(card);

        let play_state = &mut self.play_state;
        play_state.play(card);
    }

    pub fn go(&mut self) {
        let play_state = &mut self.play_state;
        play_state.go();
    }
}

impl HasScoreboard for Playing {
    fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    fn scoreboard_mut(&mut self) -> &mut Scoreboard {
        &mut self.scoreboard
    }
}

impl HasRoles for Playing {
    fn roles(&self) -> &Roles {
        &self.roles
    }

    fn roles_mut(&mut self) -> &mut Roles {
        &mut self.roles
    }
}

impl HasHands for Playing {
    fn hands(&self) -> &Hands {
        &self.hands
    }

    fn hands_mut(&mut self) -> &mut Hands {
        &mut self.hands
    }
}

impl HasCrib for Playing {
    fn crib(&self) -> &Crib {
        &self.crib
    }

    fn crib_mut(&mut self) -> &mut Crib {
        &mut self.crib
    }
}

impl HasStarterCut for Playing {
    fn starter_cut(&self) -> &StarterCut {
        &self.starter_cut
    }
}

impl HasPlayState for Playing {
    fn play_state(&self) -> &PlayState {
        &self.play_state
    }

    fn play_state_mut(&mut self) -> &mut PlayState {
        &mut self.play_state
    }
}

impl HasPending for Playing {
    fn pending(&self) -> &Pending {
        &self.pending
    }

    fn pending_mut(&mut self) -> &mut Pending {
        &mut self.pending
    }
}

impl std::fmt::Display for Playing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, play_state, starter_cut, crib, pending } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Playing(
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    play_state: {play_state},
    cut: {starter_cut},
    crib: {crib},
    pending: {pending}
)"#
        )
    }
}
