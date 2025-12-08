use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum ScoreKind {
    Fifteen,
    Pair,
    Triplet,
    Quadruplet,
    Run,
    Flush,
    LastCard,
    ThirtyOne,
    HisHeels,
    Nobs,
}
