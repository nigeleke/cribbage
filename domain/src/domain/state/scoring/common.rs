use crate::{display::format_hashmap, domain::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scoring<T> {
    scores: Scores,
    roles: Roles,
    hands: Hands,
    crib: Crib,
    cut: Cut,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Scoring<T> {
    #[rustfmt::skip]
    pub const fn new(scores: Scores, roles: Roles, hands: Hands, crib: Crib, cut: Cut) -> Self {
        Self { scores, roles, hands, crib, cut, _marker: std::marker::PhantomData, }
    }

    pub fn into_parts(self) -> (Scores, Roles, Hands, Crib, Cut) {
        #[rustfmt::skip]
        let Self { scores, roles, hands, crib, cut, _marker } = self;
        (scores, roles, hands, crib, cut)
    }
}

impl<T> HasPlayers for Scoring<T> {
    fn players(&self) -> Players {
        Players::from_iter(self.hands.keys().copied())
    }
}

impl<T> HasScores for Scoring<T> {
    fn scores(&self) -> &Scores {
        &self.scores
    }
}

impl<T> HasRoles for Scoring<T> {
    fn roles(&self) -> &Roles {
        &self.roles
    }
}

impl<T> HasHands for Scoring<T> {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl<T> HasCut for Scoring<T> {
    fn cut(&self) -> Cut {
        self.cut
    }
}

impl<T> HasCrib for Scoring<T> {
    fn crib(&self) -> &Crib {
        &self.crib
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
        let Self { scores, roles, hands, crib, cut, _marker } = self;
        let hands = format_hashmap(hands);

        write!(
            f,
            r#"Scoring{name}(
    scores: {scores},
    roles: {roles},
    hands: {hands},
    cut: {cut},
    crib: {crib}
)"#
        )
    }
}
