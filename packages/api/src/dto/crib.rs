use super::CardDTO;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CribDTO {
    pub starter_cut: Option<CardDTO>,
    pub cards: Vec<CardDTO>,
}
