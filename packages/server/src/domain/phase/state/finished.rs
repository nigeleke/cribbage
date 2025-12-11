use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{
        Crib, Hands, HasCrib, HasHands, HasRoles, HasScoreboard, HasStarterCut, Player, Roles,
        Scoreboard, StarterCut,
    },
};

/// Represents the terminal state of a completed game.
///
/// This structure captures all information required to understand the final
/// outcome, including the winning player, the final scoreboard, the roles
/// assigned during the round, the hands as they existed at the end of play,
/// the crib, and the starter card revealed during the play phase.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finished {
    winner: Player,
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter_cut: StarterCut,
}

impl Finished {
    /// Creates a new `Finished` state capturing the game’s final outcome.
    ///
    /// All parameters must reflect valid final-game state. This constructor
    /// does not perform validation; callers are responsible for ensuring
    /// coherence (e.g., that `winner` matches the final scoreboard).
    pub const fn new(
        winner: Player,
        scoreboard: Scoreboard,
        roles: Roles,
        hands: Hands,
        crib: Crib,
        starter_cut: StarterCut,
    ) -> Self {
        Self {
            winner,
            scoreboard,
            roles,
            hands,
            crib,
            starter_cut,
        }
    }

    /// Returns the player who won the game.
    #[must_use]
    pub const fn winner(&self) -> Player {
        self.winner
    }
}

impl HasScoreboard for Finished {
    fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    fn scoreboard_mut(&mut self) -> &mut Scoreboard {
        &mut self.scoreboard
    }
}

impl HasRoles for Finished {
    fn roles(&self) -> &Roles {
        &self.roles
    }

    fn roles_mut(&mut self) -> &mut Roles {
        &mut self.roles
    }
}

impl HasHands for Finished {
    fn hands(&self) -> &Hands {
        &self.hands
    }

    fn hands_mut(&mut self) -> &mut Hands {
        &mut self.hands
    }
}

impl HasCrib for Finished {
    fn crib(&self) -> &Crib {
        &self.crib
    }

    fn crib_mut(&mut self) -> &mut Crib {
        &mut self.crib
    }
}

impl HasStarterCut for Finished {
    fn starter_cut(&self) -> &StarterCut {
        &self.starter_cut
    }
}

impl std::fmt::Display for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[rustfmt::skip]
        let Self { winner, scoreboard, roles, hands, crib, starter_cut } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Finished(
    winner: {winner},
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    crib: {crib},
    cut: {starter_cut}
)"#
        )
    }
}
