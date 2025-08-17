use crate::{display::format_vec, domain::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scoring<T> {
    scoreboard: ScoreBoard,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    cut: Cut,
    pending_status: PendingStatus,
    _marker: std::marker::PhantomData<T>,
}

impl<T: PartialEq + Eq> Scoring<T> {
    #[rustfmt::skip]
    pub const fn new(scoreboard: ScoreBoard, roles: Roles, hands: Hands, crib: Crib, cut: Cut, pending_status: PendingStatus) -> Self {
        Self { scoreboard, roles, hands, crib, cut, pending_status, _marker: std::marker::PhantomData, }
    }

    pub fn into_parts(self) -> (ScoreBoard, Roles, Hands, Crib, Cut, PendingStatus) {
        #[rustfmt::skip]
        let Self { scoreboard, roles, hands, crib, cut, pending_status, _marker } = self;
        (scoreboard, roles, hands, crib, cut, pending_status)
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
        let Self { scoreboard, roles, hands, crib, cut, pending_status, _marker } = self;
        let hands = format_vec(hands);

        write!(
            f,
            r#"Scoring{name}(
    scoreboard: {scoreboard},
    roles: {roles},
    hands: {hands},
    cut: {cut},
    crib: {crib},
    pending: {pending_status}
)"#
        )
    }
}
