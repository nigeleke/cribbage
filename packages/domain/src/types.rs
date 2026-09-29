use crate::constants::CARDS_DISCARDED_TO_CRIB;
use crate::{Card, Hand, Players};

/// The two cards cut by the players to determine who deals first.
/// In Cribbage, the lower card wins the deal. Ties cause a redraw.
pub(crate) type CutsForDeal = Players<Option<Card>>;

/// Hold all players' hands.
pub(crate) type Hands = Players<Hand>;

/// The two cards discarded by a player to the crib.
pub(crate) type Discard = [Card; CARDS_DISCARDED_TO_CRIB];

/// Hold all players' discards to crib prior to forming crib.
pub(crate) type Discards = Players<Option<Discard>>;
