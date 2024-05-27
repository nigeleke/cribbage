use super::role::Role;

use std::collections::HashMap;

pub type Score = crate::domain::prelude::Score;

pub type Scores = HashMap<Role, Score>;
