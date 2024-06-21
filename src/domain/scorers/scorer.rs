use crate::types::*;

pub trait Scorer {
    fn score(&self) -> Points;
}
