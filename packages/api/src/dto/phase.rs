use serde::{Deserialize, Serialize};

/// Represents the current phase of a game for API clients.
///
/// This DTO maps the server-side game phases to a client-friendly representation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseDTO {
    /// Default phase: the game is in the lobby, waiting for players.
    #[default]
    InLobby,

    /// The phase where players are cutting the deck to determine the dealer.
    CuttingForDeal,

    /// The phase where players discard cards to the crib.
    Discarding,

    /// The phase where players take turns playing cards.
    Playing,

    /// The phase where the pone’s hand is scored.
    ScoringPone,

    /// The phase where the dealer’s hand is scored.
    ScoringDealer,

    /// The phase where the crib is scored.
    ScoringCrib,

    /// The game has finished and a winner has been determined.
    Finished,
}
