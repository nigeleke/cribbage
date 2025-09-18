use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::Hand;
use crate::display::format_vec;
use crate::{Crib, Cut, Hands, Player, Roles, Scoreboard};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finished {
    winner: Player,
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    cut: Cut,
}

impl Finished {
    #[rustfmt::skip]
    pub const fn new(winner: Player, scoreboard: Scoreboard, roles: Roles, hands: Hands, crib: Crib, cut: Cut) -> Self {
        Self { winner, scoreboard, roles, hands, crib, cut }
    }

    pub const fn winner(&self) -> Player {
        self.winner
    }

    pub fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    #[cfg(test)]
    pub fn hand(&self, player: Player) -> &Hand {
        &self.hands[player]
    }

    #[cfg(test)]
    pub fn crib(&self) -> &Crib {
        &self.crib
    }
}

impl std::fmt::Display for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[rustfmt::skip]
        let Self { winner, scoreboard, roles, hands, crib, cut } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Finished(
    winner: {winner},
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    crib: {crib},
    cut: {cut}
)"#
        )
    }
}
