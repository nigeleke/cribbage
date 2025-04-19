use std::collections::HashMap;

use super::card_stock::CardStock;
use crate::domain::Player;

/// A player's hand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandType;
pub type Hand = CardStock<HandType>;
pub type Hands = HashMap<Player, Hand>;

pub trait HasHands {
    fn hands(&self) -> &Hands;
}
