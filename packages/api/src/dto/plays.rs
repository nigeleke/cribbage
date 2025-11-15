use super::{CardDTO, PlayerDTO};
use serde::{Deserialize, Serialize};

pub type PlayDTO = (PlayerDTO, CardDTO);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaysDTO {
    pub current: Vec<PlayDTO>,
    pub historic: Vec<PlayDTO>,
}
