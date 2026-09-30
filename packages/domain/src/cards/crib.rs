use crate::Card;

/// The crib is the special pile of cards discarded cards that belongs to the dealer
/// and is scored at the end of the pegging phase.
///
/// Contains 4 cards (2 from each player) in this standard six-card Cribbage.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Crib(Vec<Card>);

impl FromIterator<Card> for Crib {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl std::ops::Deref for Crib {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
