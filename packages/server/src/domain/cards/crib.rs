use super::pile::Pile;

/// Marker type for the crib pile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CribType;

/// The crib is the special pile of cards discarded cards that belongs to the dealer
/// and is scored at the end of the pegging phase.
///
/// Contains 4 cards (2 from each player) in this standard six-card Cribbage.
pub type Crib = Pile<CribType>;

/// Trait for game states that contain a crib.
pub trait HasCrib {
    /// Immutable access to the crib.
    fn crib(&self) -> &Crib;

    /// Mutable access to the crib.
    fn crib_mut(&mut self) -> &mut Crib;
}
