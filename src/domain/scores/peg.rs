use crate::types::{Points, HasPoints};

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Peg(Points);

impl Peg {
    pub fn new(points: Points) -> Self {
        Self(points)
    }
}

impl HasPoints for Peg {
    fn points(&self) -> Points {
        self.0
    }
}

impl std::fmt::Display for Peg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
