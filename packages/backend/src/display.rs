pub fn format_vec<T: std::fmt::Display>(map: &[T]) -> String {
    map.iter()
        .map(|v| format!("{v}"))
        .collect::<Vec<_>>()
        .join(", ")
}
