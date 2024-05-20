use serde::{Serialize, Deserialize};

pub(crate) type BackPeg = usize;

pub(crate) type FrontPeg = usize;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Score(BackPeg, FrontPeg);

impl Score {
    pub fn front_peg(&self) -> FrontPeg { self.1 }
    pub fn back_peg(&self) -> BackPeg { self.0 }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.0, self.1)
    }
}