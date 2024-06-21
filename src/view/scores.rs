use super::role::Role;

use std::collections::HashMap;

pub type Score = crate::domain::Score;

pub type Scores = HashMap<Role, Score>;
