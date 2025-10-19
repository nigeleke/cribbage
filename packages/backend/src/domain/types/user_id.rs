use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::constants::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self::from(Uuid::new_v4())
    }

    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
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
            |actual| insta::assert_snapshot!(actual, @"UserId(<uuid>)"),
        );
    }
}
