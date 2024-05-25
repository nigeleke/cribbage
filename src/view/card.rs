use serde::{Serialize, Deserialize};

pub type Card = crate::domain::prelude::Card;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CardSlot {
    FaceUp(Card),
    FaceDown,
    Empty,
    Placeholder,
}

pub type Cut = Card;
