use serde::{Deserialize, Serialize};

use crate::display::format_vec;
use crate::domain::{
    Card, Crib, Hands, HasCrib, HasHands, HasPlayState, HasRoles, HasScoreboard, HasStarterCut,
    PlayState, Player, Roles, ScoreBreakdown, Scoreboard, StarterCut,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Playing {
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    play_state: PlayState,
    crib: Crib,
    starter_cut: StarterCut,
}

impl Playing {
    #[rustfmt::skip]
    pub const fn new(scoreboard: Scoreboard, roles: Roles, hands: Hands, play_state: PlayState, crib: Crib, cut: StarterCut) -> Self {
        Self { scoreboard, roles, hands, play_state, crib, starter_cut: cut }
    }

    pub fn play_card(&mut self, player: Player, card: Card) {
        let hand = &mut self.hands[player];
        hand.remove(card);
        self.play_state.play(card);
        let scoreboard = &mut self.scoreboard;
        scoreboard.peg(player, &ScoreBreakdown::play_card(&self.play_state));
        if self.play_state.target_reached() {
            self.play_state.start_new_play();
        }
    }

    pub fn pass(&mut self, player: Player) {
        self.play_state.pass();
        if self.play_state.all_players_passed() {
            let scoreboard = &mut self.scoreboard;
            scoreboard.peg(player, &ScoreBreakdown::pass(&self.play_state));
            self.play_state.start_new_play();
        }
    }

    pub fn into_parts(self) -> (Scoreboard, Roles, Hands, PlayState, Crib, StarterCut) {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, play_state, crib, starter_cut: cut } = self;
        (scoreboard, roles, hands, play_state, crib, cut)
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

impl std::fmt::Display for Playing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, play_state, starter_cut, crib } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Playing(
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    play_state: {play_state},
    cut: {starter_cut},
    crib: {crib}
)"#
        )
    }
}
