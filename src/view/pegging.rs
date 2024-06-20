use super::role::Role;

use std::collections::HashMap;

pub type Pegging = crate::domain::prelude::Pegging;

pub type Peggings = HashMap<Role, Pegging>;
