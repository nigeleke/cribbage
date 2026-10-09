use crate::constants::CARDS_DISCARDED_TO_CRIB;
use crate::{card::Card, cards::Hand, players::PlayerData};

/// The two cards cut by the players to determine who deals first.
/// In Cribbage, the lower card wins the deal. Ties cause a redraw.
pub type CutsForDeal = PlayerData<Option<Card>>;

/// Hold all players' hands.
pub type Hands = PlayerData<Hand>;

/// The two cards discarded by a player to the crib.
pub type Discard = [Card; CARDS_DISCARDED_TO_CRIB];

/// Hold all players' discards to crib prior to forming crib.
pub type Discards = PlayerData<Option<Discard>>;
