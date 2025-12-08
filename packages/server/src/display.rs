pub fn format_vec<T>(map: &[T]) -> String
where
    T: std::fmt::Display,
{
    map.iter()
        .map(|v| format!("{v}"))
        .collect::<Vec<_>>()
        .join(", ")
}
