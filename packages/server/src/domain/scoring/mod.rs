mod pegging;
mod phase;
mod score_item;
mod score_kind;
mod score_sheet;

pub use pegging::Pegging;
pub use phase::Phase as ScorePhase;
pub use score_item::ScoreItem;
pub use score_kind::ScoreKind;
pub use score_sheet::ScoreSheet;

mod constants {
    pub const SCORE_HIS_HEELS: usize = 2;
    pub const SCORE_FIFTEEN: usize = 2;
    pub const SCORE_THIRTY_ONE: usize = 2;
    pub const SCORE_GO: usize = 1;
    pub const SCORE_PAIR: usize = 2;
    pub const SCORE_ROYAL_PAIR: usize = 6;
    pub const SCORE_DOUBLE_ROYAL_PAIR: usize = 12;
    pub const SCORE_NOBS: usize = 1;

    pub const MINIMUM_RUN_LENGTH: usize = 3;
}
