use super::player::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Roles {
    dealer: Player,
    pone: Player,
}

impl Roles {
    pub const fn new(dealer: Player, pone: Player) -> Self {
        Self { dealer, pone }
    }

    pub const fn dealer(&self) -> Player {
        self.dealer
    }

    pub const fn pone(&self) -> Player {
        self.pone
    }

    pub const fn swap(&mut self) {
        std::mem::swap(&mut self.dealer, &mut self.pone);
    }
}

pub trait HasRoles {
    fn roles(&self) -> &Roles;

    fn dealer(&self) -> Player {
        self.roles().dealer()
    }

    fn pone(&self) -> Player {
        self.roles().pone()
    }
}

impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Roles(dealer: {}, pone: {})", self.dealer, self.pone)
    }
}
