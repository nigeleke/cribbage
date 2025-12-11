use crate::domain::cards::pile::Pile;

/// Marker type for a player's personal hand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandType;

/// A hand is the set of cards a player holds privately during play.
/// 6 cards dealt then 4 after discarding to the crib.
pub type Hand = Pile<HandType>;
