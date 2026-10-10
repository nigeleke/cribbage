use crate::game::{
    Cutting, Dealing, Discarding, Finished, Game, Playing, ScoringCrib, ScoringDealer, ScoringPone,
};
use crate::players::{Player, Roles};

/// Provides access to the current dealer and pone.
pub trait HasRoles {
    /// Returns the current roles.
    fn roles(&self) -> &Roles;

    /// Returns the player currently assigned as dealer.
    fn dealer(&self) -> Player {
        self.roles().dealer().player()
    }

    /// Returns the player currently assigned as pone.
    fn pone(&self) -> Player {
        self.roles().pone().player()
    }
}

impl HasRoles for Dealing {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasRoles for Discarding {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasRoles for Cutting {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasRoles for Playing {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasRoles for ScoringPone {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasRoles for ScoringDealer {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasRoles for ScoringCrib {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl HasRoles for Finished {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl<T: HasRoles> Game<T> {
    /// Return the assigned roles in the current game.
    pub fn roles(&self) -> &Roles {
        self.state.roles()
    }
}

impl<T: HasRoles> HasRoles for Game<T> {
    /// Return the assigned roles in the current game.
    fn roles(&self) -> &Roles {
        self.state.roles()
    }
}
