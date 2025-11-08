use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameId(Uuid);

impl GameId {
    /// Generates a new, random `GameId` using a Uuid.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for GameId {
    fn from(value: Uuid) -> Self {
        GameId(value)
    }
}

impl std::str::FromStr for GameId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s)?;
        Ok(GameId(uuid))
    }
}

impl std::fmt::Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameId({})", self.0)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::test::filtered_assert;

//     #[test]
//     fn game_id_is_displayable() {
//         filtered_assert(
//             GameId::new().to_string(),
//             |actual| insta::assert_snapshot!(actual, @"GameId(<uuid>)"),
//         );
//     }
// }
