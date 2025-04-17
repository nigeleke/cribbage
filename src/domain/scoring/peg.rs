use serde::{Deserialize, Serialize};

use crate::domain::Points;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Peg(Points);

impl Peg {
    pub fn new(points: Points) -> Self {
        Self(points)
    }

    pub fn points(&self) -> Points {
        self.0
    }
}

impl std::fmt::Display for Peg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
