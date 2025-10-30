use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, AsRefStr)]
pub enum CutForDealStateDTO {
    Pending,
    RedrawRequired,
    DealerSelected,
}

impl std::fmt::Display for CutForDealStateDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
