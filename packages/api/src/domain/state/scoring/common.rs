use crate::{Crib, Cut, Hand, Hands, Pending, Player, Roles, Scoreboard, display::format_vec};
#[cfg(test)]
use crate::{Dealer, Pone};
use serde::{Deserialize, Serialize};

pub type WaitingForScoresViewed = Pending;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scoring<T> {
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    cut: Cut,
    pending: WaitingForScoresViewed,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Scoring<T> {
    #[rustfmt::skip]
    pub const fn new(scoreboard: Scoreboard, roles: Roles, hands: Hands, crib: Crib, cut: Cut, pending: WaitingForScoresViewed) -> Self {
        Self { scoreboard, roles, hands, crib, cut, pending, _marker: std::marker::PhantomData, }
    }

    pub fn into_parts(self) -> (Scoreboard, Roles, Hands, Crib, Cut, WaitingForScoresViewed) {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, crib, cut, pending, _marker } = self;
        (scoreboard, roles, hands, crib, cut, pending)
    }

    pub fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    #[cfg(test)]
    pub fn dealer(&self) -> &Dealer {
        self.roles.dealer()
    }

    #[cfg(test)]
    pub fn pone(&self) -> &Pone {
        self.roles.pone()
    }

    pub fn hand(&self, player: Player) -> &Hand {
        &self.hands[player]
    }

    pub fn crib(&self) -> &Crib {
        &self.crib
    }

    pub fn cut(&self) -> Cut {
        self.cut
    }

    pub fn pending(&self) -> &WaitingForScoresViewed {
        &self.pending
    }
}

impl<T> std::fmt::Display for Scoring<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = std::any::type_name::<T>();
        let name = name.rsplit("::").next().expect("name.rsplit.next");
        let name = name
            .strip_prefix("Scoring")
            .and_then(|s| s.strip_suffix("Type"))
            .unwrap_or(name);

        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, crib, cut, pending, _marker } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Scoring{name}(
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    cut: {cut},
    crib: {crib},
    pending: {pending}
)"#
        )
    }
}
