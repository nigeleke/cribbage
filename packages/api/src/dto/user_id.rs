use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data transfer object for a user identifier, wrapping a UUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserIdDTO(Uuid);

impl UserIdDTO {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(self) -> Uuid {
        self.0
    }
}

impl Default for UserIdDTO {
    fn default() -> Self {
        Self(Uuid::nil())
    }
}

impl std::fmt::Display for UserIdDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// #[cfg(test)]
// mod test {
//     use serde::Serialize;

//     use crate::UserIdDTO;

//     #[test]
//     fn user_id_can_be_serialized() {
//         let user_id_0 = UserIdDTO::new();
//         println!("{user_id_0}");
//         let user_id_1 = user_id_0.\;
//         println!("{user_id_1:?}");
//         todo!()
//     }
// }
