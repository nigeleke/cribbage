use super::card::{Card, CardSlot};
use super::player::Role;

use std::collections::HashMap;

pub type Cuts = HashMap<Role, Card>;

pub type Hand = Vec<CardSlot>;

pub type Hands = HashMap<Role, Hand>;

pub type Crib = Vec<CardSlot>;
