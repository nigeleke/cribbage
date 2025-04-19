use std::collections::HashMap;
use std::fmt::Display;

pub fn format_hashmap<P, T>(map: &HashMap<P, T>) -> String
where
    P: Display,
    T: Display,
{
    map.iter()
        .map(|(k, v)| format!("{} -> {}", k, v))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn format_vec<T: Display>(map: &[T]) -> String {
    map.iter()
        .map(|v| format!("{}", v))
        .collect::<Vec<_>>()
        .join(", ")
}
