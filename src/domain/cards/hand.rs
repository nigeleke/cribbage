use super::cards::Cards;

use crate::types::*;

use std::collections::HashMap;

/// A player's hand.
#[derive(Clone, Debug, PartialEq)]
pub struct HandType;
pub type Hand = Cards<HandType>;
pub type Hands = HashMap<Player, Hand>;
