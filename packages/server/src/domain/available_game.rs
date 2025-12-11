use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::domain::GameId;

/// How a game appears in the public lobby / matchmaking pool.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString)]
pub enum Availability {
    /// Only joinable via direct invite or link. Does not appear in public list.
    Private,

    /// Visible and joinable by anyone in the lobby browser (other than the original host).
    Public,
}

/// A game that is visible in the public lobby and can be joined. These only get created
/// with respect to a given user, i.e. they are "available" to that user. Either the user
/// is the host, or participating in the game, or they can join as a guest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableGame {
    id: GameId,
    name: String,
    availability: Availability,
}

impl AvailableGame {
    /// Creates a new game entry for the lobby, available for a given user.
    #[must_use]
    pub fn new(id: GameId, name: String, availability: Availability) -> Self {
        Self {
            id,
            name,
            availability,
        }
    }

    /// Returns the unique game identifier.
    #[inline(always)]
    #[must_use]
    pub fn id(&self) -> &GameId {
        &self.id
    }

    /// Returns the human-readable name of the game.
    #[inline(always)]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the visibility setting of the game.
    #[inline(always)]
    #[must_use]
    pub fn availability(&self) -> &Availability {
        &self.availability
    }
}
