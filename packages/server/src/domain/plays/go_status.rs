use serde::{Deserialize, Serialize};
use strum::AsRefStr;

/// Represents the status of a "go" during pegging.
///
/// Tracks whether a go has been called or if play has continued after a go.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum GoStatus {
    #[doc(hidden)]
    #[default]
    NotCalled,

    #[doc(hidden)]
    Called,

    #[doc(hidden)]
    PlayContinued,
}
