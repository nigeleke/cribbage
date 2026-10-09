/// Represents the status of a "go" during pegging.
///
/// Tracks whether a go has been called or if play has continued after a go.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[rustfmt::skip]
pub enum GoStatus {
    #[doc(hidden)] #[default] NotCalled,
    #[doc(hidden)] Called,
    #[doc(hidden)] Continued,
}

impl std::fmt::Debug for GoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GoStatus::NotCalled => "not-called",
            GoStatus::Called => "called",
            GoStatus::Continued => "continued",
        })
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;

    #[test]
    fn has_debug_text() {
        let values = [GoStatus::NotCalled, GoStatus::Called, GoStatus::Continued];
        let expected = ["not-called", "called", "continued"];
        values
            .iter()
            .map(|s| format!("{s:?}"))
            .zip(expected.iter())
            .for_each(|(actual, expected)| assert_eq!(actual, expected.to_string()));
    }
}
