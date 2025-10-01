use super::role::Role;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type Pegging = crate::domain::Pegging;

pub type Peggings = HashMap<Role, Pegging>;

pub type ScoreReasons = crate::domain::ScoreReasons;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Scores(Peggings, ScoreReasons);

impl Scores {
    pub fn new(peggings: Peggings, reasons: ScoreReasons) -> Self {
        Self(peggings, reasons)
    }

    pub fn peggings(&self) -> Peggings {
        self.0.clone()
    }
}
