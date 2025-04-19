use super::card::Card;

use crate::domain::Player;

use std::collections::HashMap;

pub type Cut = Card;

pub trait HasCut {
    fn cut(&self) -> Cut;
}

pub type Cuts = HashMap<Player, Cut>;

pub trait HasCuts {
    fn cuts(&self) -> &Cuts;
}
