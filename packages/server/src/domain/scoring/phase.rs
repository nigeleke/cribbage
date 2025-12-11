use serde::{Deserialize, Serialize};

/// Represents the distinct phases of the game.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    #[doc(hidden)]
    Starter,

    #[doc(hidden)]
    Play,

    #[doc(hidden)]
    PoneHand,

    #[doc(hidden)]
    DealerHand,

    #[doc(hidden)]
    Crib,
}
