use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct BackPeg(usize);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct FrontPeg(usize);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Score(BackPeg, FrontPeg);

impl Score {
    pub(crate) fn new() -> Self {
        Self(BackPeg(0), FrontPeg(0))
    }
}