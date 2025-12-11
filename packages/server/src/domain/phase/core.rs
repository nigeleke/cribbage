use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::domain::{
    Discarding, Finished, Playing, ScoringCrib, ScoringDealer, ScoringPone, Starting,
    phase::wrap::{Wrap, WrapOrFinished},
};

/// Current phase of the Cribbage game.
///
/// The game progresses through a strict sequence of phases, each represented
/// by a variant containing the phase-specific data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum Phase {
    /// - `Starting` – Game is being created, players are joining and cutting for first dealer.
    Starting(Starting),

    /// - `Discarding` – Players discard two cards each to form the crib
    Discarding(Discarding),

    /// - `Playing` – Players alternately play cards; pegging occurs
    Playing(Playing),

    /// - `ScoringPone` – Pone’s hand is scored
    ScoringPone(ScoringPone),

    /// - `ScoringDealer` – Dealer’s hand is scored
    ScoringDealer(ScoringDealer),

    /// - `ScoringCrib` – Dealer’s crib is scored
    ScoringCrib(ScoringCrib),

    /// - `Finished` – Game over, final scores determined
    Finished(Finished),
}

impl Phase {
    pub(crate) fn or_finished(self) -> Phase {
        match self {
            Phase::Starting(s) => s.wrap(),
            Phase::Discarding(s) => s.wrap(),
            Phase::Playing(s) => s.wrap_or_finished(),
            Phase::ScoringPone(s) => s.wrap_or_finished(),
            Phase::ScoringDealer(s) => s.wrap_or_finished(),
            Phase::ScoringCrib(s) => s.wrap_or_finished(),
            Phase::Finished(s) => s.wrap(),
        }
    }
}

impl Default for Phase {
    fn default() -> Self {
        let starting = Starting::default();
        starting.wrap()
    }
}

impl std::fmt::Display for Phase {
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
