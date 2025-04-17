use crate::domain::{Crib, Cut, Hands, Roles, Scores};

#[derive(Debug)]
pub struct Scoring {
    scores: Scores,
    roles: Roles,
    hands: Hands,
    cut: Cut,
    crib: Crib,
}

impl Scoring {
    pub fn new(scores: Scores, roles: Roles, hands: Hands, cut: Cut, crib: Crib) -> Self {
        Self {
            scores,
            roles,
            hands,
            cut,
            crib,
        }
    }

    pub fn into_parts(self) -> (Scores, Roles, Hands, Cut, Crib) {
        let Self {
            scores,
            roles,
            hands,
            cut,
            crib,
        } = self;
        (scores, roles, hands, cut, crib)
    }
}
