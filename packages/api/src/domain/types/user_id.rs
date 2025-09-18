use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::constants::*;

/// A unique identifier for a user, internally represented as a ULID.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(Ulid);

impl UserId {
    /// Generates a new, random `UserId` using a ULID.
    pub fn new() -> Self {
        Self::from(Ulid::new())
    }
}

impl From<Ulid> for UserId {
    fn from(value: Ulid) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UserId({})", self.0)
    }
}

pub type Users = [UserId; PLAYER_COUNT];

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::filtered_assert;

    #[test]
    fn user_id_is_displayable() {
        filtered_assert(
            UserId::new().to_string(),
            |actual| insta::assert_snapshot!(actual, @"<userid>"),
        );
    }
}
