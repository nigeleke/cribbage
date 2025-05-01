use std::collections::HashMap;

use super::card::Card;
use crate::domain::Player;

pub type Cut = Card;

pub trait HasCut {
    fn cut(&self) -> Cut;
}

pub type Cuts = HashMap<Player, Cut>;
