#[cfg(test)]
use crate::Pone;
use crate::{
    Crib, Cut, Dealer, Event, EventKind, GameId, Hand, Hands, PlayState, Player, Roles,
    ScoreBreakdown, Scoreboard, display::format_vec,
};
use eventsourced::EventSourced;
use serde::{Deserialize, Serialize};

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

impl EventSourced for Playing {
    type Id = GameId;
    type Event = Event;

    const TYPE_NAME: &'static str = stringify!(Playing);

    fn handle_event(mut self, event: Self::Event) -> Self {
        match event.kind() {
            EventKind::StarterCardCut { cut } => {
                let player = self.dealer().player();
                let scoreboard = &mut self.scoreboard;
                scoreboard.peg(player, &ScoreBreakdown::his_heels(*cut));
            }
            EventKind::CardPlayed { player, card } => {
                let hand = &mut self.hands[*player];
                hand.remove(*card);
                self.play_state.play(*card);
                let scoreboard = &mut self.scoreboard;
                scoreboard.peg(*player, &ScoreBreakdown::play_card(&self.play_state));
                if self.play_state.target_reached() {
                    self.play_state.start_new_play();
                }
            }
            EventKind::Passed { player } => {
                self.play_state.pass();
                if self.play_state.all_players_passed() {
                    let scoreboard = &mut self.scoreboard;
                    scoreboard.peg(*player, &ScoreBreakdown::pass(&self.play_state));
                    self.play_state.start_new_play();
                }
            }
            _ => {}
        }
        self
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
