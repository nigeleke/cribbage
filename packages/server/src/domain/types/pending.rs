use std::collections::HashSet;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::domain::{PLAYER0, PLAYER1, Player};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pending(HashSet<Player>);

pub trait HasPending {
    fn pending(&self) -> &Pending;
    fn pending_mut(&mut self) -> &mut Pending;
}

impl Pending {
    pub fn finished(&self) -> bool {
        self.0.is_empty()
    }

    pub fn waiting_on(&self, player: Player) -> bool {
        self.0.contains(&player)
    }

    pub fn acknowledge(&mut self, player: Player) -> bool {
        self.0.remove(&player);
        self.0.is_empty()
    }
}

impl Default for Pending {
    fn default() -> Self {
        Self(HashSet::from([PLAYER0, PLAYER1]))
    }
}

#[cfg(test)]
impl From<&[Player]> for Pending {
    fn from(value: &[Player]) -> Self {
        Self(HashSet::from_iter(value.to_owned()))
    }
}

impl std::fmt::Display for Pending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pending = self.0.iter().map(|p| p.to_string()).join(", ");
        write!(f, "Pending({pending})")
    }
}
