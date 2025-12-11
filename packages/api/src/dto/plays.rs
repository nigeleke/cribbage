use serde::{Deserialize, Serialize};

use super::{CardDTO, CardIdDTO, PlayerDTO};

/// A tuple representing a single play in the game: the player and the card they played.
pub type PlayDTO = (PlayerDTO, CardDTO);

/// Represents the possible actions a player can take during a turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayActionDTO {
    /// The specified player plays a card.
    Play(PlayerDTO),

    /// The specified player calls "Go".
    Go(PlayerDTO),

    /// The pone's hand is being scored.
    ScorePone,
}

/// Represents the current state of plays during a round for API clients.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaysDTO {
    /// The action that the next player is expected to take.
    pub next_action: PlayActionDTO,

    /// The list of legal card IDs that the current user can play.
    /// Will be empty if the next action is on the opponent.
    pub legal_plays: Vec<CardIdDTO>,

    /// The plays made so far in the current round.
    pub current: Vec<PlayDTO>,

    /// The plays made in previous rounds.
    pub previous: Vec<PlayDTO>,

    /// The running total of points in the current play sequence.
    pub running_total: u8,
}
