use crate::domain::game::ScoreComposition;

pub trait ScoringRule {
    fn score(&self) -> ScoreComposition;
}
