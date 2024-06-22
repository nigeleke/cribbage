use crate::domain::ScoreReasons;

pub trait Scorer {
    fn score(&self) -> ScoreReasons;
}
