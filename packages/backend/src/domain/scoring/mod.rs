mod breakdown;
mod event;
mod kind;
mod phase;

pub use breakdown::Breakdown as ScoreBreakdown;
pub use event::Event as ScoreEvent;
pub use kind::Kind as ScoreKind;
pub use phase::Phase as ScorePhase;

mod constants {
    pub const SCORE_HIS_HEELS: usize = 2;
    pub const SCORE_FIFTEEN: usize = 2;
    pub const SCORE_THIRTY_ONE: usize = 2;
    pub const SCORE_UNDER_THIRTY_ONE: usize = 1;
    pub const SCORE_PAIR: usize = 2;
    pub const SCORE_ROYAL_PAIR: usize = 6;
    pub const SCORE_DOUBLE_ROYAL_PAIR: usize = 12;
    pub const SCORE_NOBS: usize = 1;

    pub const MINIMUM_RUN_LENGTH: usize = 3;
}
