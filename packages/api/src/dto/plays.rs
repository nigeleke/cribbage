use super::{CardDTO, CardIdDTO, PlayerDTO};
use serde::{Deserialize, Serialize};

pub type PlayDTO = (PlayerDTO, CardDTO);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayActionDTO {
    Play(PlayerDTO),
    Pass(PlayerDTO),
    ScorePone,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaysDTO {
    pub next_action: PlayActionDTO,
    pub legal_plays: Vec<CardIdDTO>,
    pub current: Vec<PlayDTO>,
    pub historic: Vec<PlayDTO>,
}
