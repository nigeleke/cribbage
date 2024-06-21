use serde::{Serialize, Deserialize};

pub type Card = crate::domain::Card;

#[derive(Clone, Copy, Default, Serialize, Deserialize, Debug)]
pub enum CardSlot {
    FaceUp(Card),
    FaceDown,
    #[default]
    Empty,
    Placeholder,
}

pub type Cut = Card;
