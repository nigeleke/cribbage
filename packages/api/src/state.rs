use crate::{Discarding, Finished, Playing, ScoringCrib, ScoringDealer, ScoringPone, Starting};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum State {
    Starting(Starting),
    Discarding(Discarding),
    Playing(Playing),
    ScoringPone(ScoringPone),
    ScoringDealer(ScoringDealer),
    ScoringCrib(ScoringCrib),
    Finished(Finished),
}

impl Default for State {
    fn default() -> Self {
        let starting = Starting::default();
        State::Starting(starting)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Starting(state) => state.fmt(f),
            Self::Discarding(state) => state.fmt(f),
            Self::Playing(state) => state.fmt(f),
            Self::ScoringPone(state) => state.fmt(f),
            Self::ScoringDealer(state) => state.fmt(f),
            Self::ScoringCrib(state) => state.fmt(f),
            Self::Finished(state) => state.fmt(f),
        }
    }
}
