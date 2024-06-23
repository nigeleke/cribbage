use super::role::Role;

use std::collections::HashMap;

pub type Pegging = crate::domain::Pegging;

pub type Peggings = HashMap<Role, Pegging>;
