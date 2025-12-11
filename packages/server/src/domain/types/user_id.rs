use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifier for a user/account.
///
/// # Examples
///
/// ```
/// # use my_crate::UserId;
/// let id = UserId::new();
/// let id2 = UserId::nil();
///
/// assert_ne!(id, id2);
/// assert_eq!(id.to_string().len(), 36);
/// println!("User joined: {}", id);           // Display works automatically
/// ```
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct UserId(Uuid);

impl UserId {
    /// Generates a new random version-4 UUID.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::from(Uuid::new_v4())
    }

    /// Returns the inner [`Uuid`].
    #[inline]
    #[must_use]
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

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::test::filtered_assert;

//     #[test]
//     fn user_id_is_displayable() {
//         filtered_assert(
//             UserId::new().to_string(),
//             |actual| insta::assert_snapshot!(actual, @"UserId(<uuid>)"),
//         );
//     }
// }
