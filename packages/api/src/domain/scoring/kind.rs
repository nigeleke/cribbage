use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    HisHeels,
    Fifteen,
    Pair,
    Triplet,
    Quadruplet,
    Run,
    Flush,
    HisNobs,
}
