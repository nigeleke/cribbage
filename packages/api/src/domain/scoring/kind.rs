use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    HisNobs,
}
