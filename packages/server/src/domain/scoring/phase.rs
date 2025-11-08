use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Starter,
    Play,
    PoneHand,
    DealerHand,
    Crib,
}
