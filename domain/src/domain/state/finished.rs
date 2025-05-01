use crate::{
    display::format_hashmap,
    domain::{Crib, Cut, Hands, HasCut, HasScores, Roles, Scores},
};

#[derive(Debug)]
pub struct Finished {
    pub scores: Scores,
    pub roles: Roles,
    pub hands: Hands,
    pub crib: Crib,
    pub cut: Cut,
}

impl HasScores for Finished {
    fn scores(&self) -> &Scores {
        &self.scores
    }
}

impl HasCut for Finished {
    fn cut(&self) -> Cut {
        self.cut
    }
}

impl std::fmt::Display for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Finished(
    scores: {},
    roles: {},
    hands: {},
    crib: {},
    cut: {}
)"#,
            self.scores,
            self.roles,
            format_hashmap(&self.hands),
            self.crib,
            self.cut
        )
    }
}
