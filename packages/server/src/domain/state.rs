mod discarding;
mod finished;
mod playing;
mod scoring;
mod starting;

pub mod exports {
    pub use super::{
        discarding::Discarding,
        finished::Finished,
        playing::Playing,
        scoring::{ScoringCrib, ScoringDealer, ScoringPone},
        starting::Starting,
    };
}

pub use exports::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::domain::{HasCrib, HasHands, HasRoles, HasScoreboard, HasStarterCut};

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

impl State {
    pub fn or_finished(self) -> State {
        match self {
            State::Starting(s) => s.wrap(),
            State::Discarding(s) => s.wrap(),
            State::Playing(s) => s.wrap_or_finished(),
            State::ScoringPone(s) => s.wrap_or_finished(),
            State::ScoringDealer(s) => s.wrap_or_finished(),
            State::ScoringCrib(s) => s.wrap_or_finished(),
            State::Finished(s) => s.wrap(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let starting = Starting::default();
        starting.wrap()
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

pub trait Wrap {
    fn wrap(self) -> State;
}

macro_rules! impl_wrap {
    ($($ty:ident => $variant:ident),* $(,)?) => {
        $(
            impl Wrap for $ty {
                fn wrap(self) -> State {
                    State::$variant(self)
                }
            }
        )*
    };
}

impl_wrap! {
    Starting      => Starting,
    Discarding    => Discarding,
    Playing       => Playing,
    ScoringPone   => ScoringPone,
    ScoringDealer => ScoringDealer,
    ScoringCrib   => ScoringCrib,
    Finished      => Finished,
}

trait MaybeFinished {
    fn wrap_or_finished(self) -> State;
}

impl<T> MaybeFinished for T
where
    T: HasScoreboard + HasRoles + HasHands + HasCrib + HasStarterCut + Wrap,
{
    fn wrap_or_finished(self) -> State {
        if let Some(winner) = self.scoreboard().winner() {
            let finished = Finished::new(
                winner,
                self.scoreboard().clone(),
                *self.roles(),
                self.hands().clone(),
                self.crib().clone(),
                *self.starter_cut(),
            );
            finished.wrap()
        } else {
            self.wrap()
        }
    }
}
