use crate::Card;

/// A hand is the set of cards a player holds privately during play.
/// 6 cards dealt then 4 after discarding to the crib.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Hand(Vec<Card>);

impl Hand {
    pub(crate) fn remove(&mut self, card: Card) {
        self.0.retain(|c| c != &card);
    }

    pub(crate) fn remove_all(&mut self, cards: &[Card]) {
        self.0.retain(|c| !cards.contains(c));
    }
}

impl FromIterator<Card> for Hand {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl std::ops::Deref for Hand {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
