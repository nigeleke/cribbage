use super::{CardDTO, PlayerDTO};
use serde::{Deserialize, Serialize};

pub type PlayDTO = (PlayerDTO, CardDTO);

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaysDTO {
    pub current: Vec<PlayDTO>,
    pub historic: Vec<PlayDTO>,
}
