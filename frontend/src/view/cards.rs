use super::card::{Card, CardSlot};
use super::role::Role;

use std::collections::HashMap;

pub type Cut = Card;

pub type Cuts = HashMap<Role, Cut>;

pub type Hand = Vec<CardSlot>;

pub type Hands = HashMap<Role, Hand>;

pub type Crib = Vec<CardSlot>;
