use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum Kind {
    Fifteen,
    Pair,
    Triplet,
    Quadruplet,
    Run,
    Flush,
    Go,
    ThirtyOne,
    HisHeels,
    Nobs,
}
