pub use crate::{Card, Hand, Players};

/// The two cards cut by the players to determine who deals first.
/// In Cribbage, the lower card wins the deal. Ties cause a redraw.
pub type CutsForDeal = Players<Option<Card>>;

/// Hold all players' hands.
pub type Hands = Players<Hand>;

/// The two cards discarded by a player to the crib.
pub type Discard = [Card; 2];

/// Hold all players' discards to crib prior to forming crib.
pub type Discards = Players<Option<Discard>>;
