use super::card_stock::CardStock;

use crate::domain::Player;

use std::collections::HashMap;

/// A player's hand.
#[derive(Clone, Debug, PartialEq)]
pub struct HandType;
pub type Hand = CardStock<HandType>;
pub type Hands = HashMap<Player, Hand>;

pub trait HasHands {
    fn hands(&self) -> &Hands;
}
