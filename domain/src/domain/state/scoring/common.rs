use crate::{
    display::format_hashmap,
    domain::{
        Crib, Cut, Hands, HasCrib, HasCut, HasHands, HasPlayers, HasRoles, HasScores, Players,
        Roles, Scores,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scoring<T> {
    pub scores: Scores,
    pub roles: Roles,
    pub hands: Hands,
    pub crib: Crib,
    pub cut: Cut,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Scoring<T> {
    #[rustfmt::skip]
    pub const fn new(scores: Scores, roles: Roles, hands: Hands, crib: Crib, cut: Cut) -> Self {
        Self { scores, roles, hands, crib, cut, _marker: std::marker::PhantomData, }
    }

    pub fn into_parts(self) -> (Scores, Roles, Hands, Crib, Cut) {
        (self.scores, self.roles, self.hands, self.crib, self.cut)
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

        write!(
            f,
            r#"Scoring{}(
    scores: {},
    roles: {},
    hands: {},
    cut: {},
    crib: {}
)"#,
            name,
            self.scores,
            self.roles,
            format_hashmap(&self.hands),
            self.cut,
            self.crib
        )
    }
}
