use serde::{Deserialize, Serialize};

use super::{CardDTO, CardIdDTO, PlayerDTO};

pub type PlayDTO = (PlayerDTO, CardDTO);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayActionDTO {
    Play(PlayerDTO),
    Go(PlayerDTO),
    ScorePone,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaysDTO {
    pub next_action: PlayActionDTO,
    pub legal_plays: Vec<CardIdDTO>,
    pub current: Vec<PlayDTO>,
    pub previous: Vec<PlayDTO>,
    pub running_total: u8,
}
