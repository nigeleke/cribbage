use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum GoStatus {
    #[default]
    NotCalled,
    Called,
    PlayContinued,
}
