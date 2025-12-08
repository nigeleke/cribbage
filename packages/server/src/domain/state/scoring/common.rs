use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{
        Crib, Hands, HasCrib, HasHands, HasPending, HasRoles, HasScoreboard, HasStarterCut,
        Pegging, Pending, Roles, Scoreboard, StarterCut,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scoring<T> {
    scoreboard: Scoreboard,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    starter_cut: StarterCut,
    pegging: Pegging,
    pending: Pending,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Scoring<T> {
    pub const fn new(
        scoreboard: Scoreboard,
        roles: Roles,
        hands: Hands,
        crib: Crib,
        starter_cut: StarterCut,
        pegging: Pegging,
        pending: Pending,
    ) -> Self {
        Self {
            scoreboard,
            roles,
            hands,
            crib,
            starter_cut,
            pegging,
            pending,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn pegging(&self) -> &Pegging {
        &self.pegging
    }
}

impl<T> HasScoreboard for Scoring<T> {
    fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    fn scoreboard_mut(&mut self) -> &mut Scoreboard {
        &mut self.scoreboard
    }
}

impl<T> HasRoles for Scoring<T> {
    fn roles(&self) -> &Roles {
        &self.roles
    }

    fn roles_mut(&mut self) -> &mut Roles {
        &mut self.roles
    }
}

impl<T> HasHands for Scoring<T> {
    fn hands(&self) -> &Hands {
        &self.hands
    }

    fn hands_mut(&mut self) -> &mut Hands {
        &mut self.hands
    }
}

impl<T> HasCrib for Scoring<T> {
    fn crib(&self) -> &Crib {
        &self.crib
    }

    fn crib_mut(&mut self) -> &mut Crib {
        &mut self.crib
    }
}

impl<T> HasStarterCut for Scoring<T> {
    fn starter_cut(&self) -> &StarterCut {
        &self.starter_cut
    }
}

impl<T> HasPending for Scoring<T> {
    fn pending(&self) -> &Pending {
        &self.pending
    }

    fn pending_mut(&mut self) -> &mut Pending {
        &mut self.pending
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
        let Self { scoreboard, roles, hands, crib, starter_cut: cut, pegging, pending, _marker } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Scoring{name}(
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    cut: {cut},
    crib: {crib},
    pegging: {pegging},
    pending: {pending}
)"#
        )
    }
}
