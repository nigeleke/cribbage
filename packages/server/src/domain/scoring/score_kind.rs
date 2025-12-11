use serde::{Deserialize, Serialize};
use strum::AsRefStr;

/// Represents the kind of scoring event that occurred during play.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum ScoreKind {
    #[doc(hidden)]
    Fifteen,

    #[doc(hidden)]
    Pair,

    #[doc(hidden)]
    Triplet,

    #[doc(hidden)]
    Quadruplet,

    #[doc(hidden)]
    Run,

    #[doc(hidden)]
    Flush,

    #[doc(hidden)]
    LastCard,

    #[doc(hidden)]
    ThirtyOne,

    #[doc(hidden)]
    HisHeels,

    #[doc(hidden)]
    Nobs,
}
