use super::card::Card;

use crate::types::prelude::Player;

use std::collections::HashMap;

pub type Cut = Card;

pub type Cuts = HashMap<Player, Cut>;
