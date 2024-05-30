use super::player::Player;

use std::collections::HashMap;
use std::fmt::Display;

pub(crate) fn format_hashmap<T: Display>(map: &HashMap<Player, T>) -> String {
    map.iter().map(|(k, v)| format!("{} -> {}", k, v)).collect::<Vec<_>>().join(", ")
}

pub(crate) fn format_vec<T: Display>(map: &[T]) -> String {
    map.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(", ")
}
