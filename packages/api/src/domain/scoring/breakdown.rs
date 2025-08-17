use super::constants::*;
use crate::{Card, Points, ScoreEvent, ScoreKind};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Breakdown(Vec<ScoreEvent>);

impl Breakdown {
    pub fn his_heels(self, cut: Card) -> Self {
        let event = ScoreEvent::new(
            ScoreKind::HisHeels,
            vec![cut],
            Points::from(SCORE_HIS_HEELS),
        );

        Self(vec![event])
    }

    pub fn points(&self) -> Points {
        self.0.iter().map(ScoreEvent::points).sum()
    }
}
