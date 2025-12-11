use thiserror::*;

/// Errors that occur when manipulating cards in piles, hands, crib, etc.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum CardsError {
    /// The string could not be parsed into a valid `Card`.
    /// Example: `"X5"`, `"14H"`, `"Ah"` (invalid suit), empty string.
    #[error("invalid card")]
    InvalidCard(String),

    /// A requested card is not present in the pile/hand/deck.
    #[error("card {0} not found")]
    CardNotFound(String),

    /// Not enough cards are available to fulfill a cut or deal request.
    #[error("cannot take cards required {0} of cards")]
    CardsNotAvailable(u8),
}
