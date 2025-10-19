use serde::{Deserialize, Serialize};

use crate::display::format_vec;
#[cfg(test)]
use crate::domain::Pone;
use crate::domain::{
    Card, Crib, Cut, Dealer, Hand, Hands, PlayState, Player, Roles, ScoreBreakdown, Scoreboard,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Playing {
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    play_state: PlayState,
    crib: Crib,
    cut: Cut,
}

impl Playing {
    #[rustfmt::skip]
    pub const fn new(scoreboard: Scoreboard, roles: Roles, hands: Hands, play_state: PlayState, crib: Crib, cut: Cut) -> Self {
        Self { scoreboard, roles, hands, play_state, crib, cut }
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

    pub fn into_parts(self) -> (Scoreboard, Roles, Hands, PlayState, Crib, Cut) {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, play_state, crib, cut } = self;
        (scoreboard, roles, hands, play_state, crib, cut)
    }

    pub fn hand(&self, player: Player) -> &Hand {
        &self.hands[player]
    }

    #[cfg(test)]
    pub fn crib(&self) -> &Crib {
        &self.crib
    }

    #[cfg(test)]
    pub fn cut(&self) -> Cut {
        self.cut
    }

    pub fn play_state(&self) -> &PlayState {
        &self.play_state
    }

    pub fn dealer(&self) -> &Dealer {
        self.roles.dealer()
    }

    #[cfg(test)]
    pub fn pone(&self) -> &Pone {
        self.roles.pone()
    }

    pub fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }
}

impl std::fmt::Display for Playing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, play_state, cut, crib } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Playing(
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    play_state: {play_state},
    cut: {cut},
    crib: {crib}
)"#
        )
    }
}
