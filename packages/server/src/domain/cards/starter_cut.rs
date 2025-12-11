pub use crate::domain::Card;

/// The card turned up after the deal as the "starter".
pub type StarterCut = Card;

/// Trait for game states that have a starter card.
pub trait HasStarterCut {
    /// Returns the current starter card.
    fn starter_cut(&self) -> &StarterCut;
}
