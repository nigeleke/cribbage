use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// A unique identifier for a game, internally represented as a ULID.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameId(Ulid);

impl GameId {
    /// Generates a new, random `GameId` using a ULID.
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl std::fmt::Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameId({})", self.0)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::test::filtered_assert;

    #[test]
    fn game_id_is_displayable() {
        filtered_assert(
            GameId::new().to_string(),
            |actual| insta::assert_snapshot!(actual, @"<gameid>"),
        );
    }
}
