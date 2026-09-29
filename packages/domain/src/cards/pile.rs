use rand::prelude::{Rng, SliceRandom};

use crate::Card;

#[derive(Clone, PartialEq, Eq)]
pub struct Pile<T> {
    cards: Vec<Card>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Pile<T> {
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_all(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    pub fn remove(&mut self, card: Card) {
        self.cards.retain(|c| c != &card);
    }

    pub fn remove_all(&mut self, cards: &[Card]) {
        self.cards.retain(|c| !cards.contains(c));
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn take(&mut self, n: usize) -> Vec<Card> {
        self.cards.drain(..n).collect()
    }

    pub fn contains_all(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| self.cards.contains(c))
    }

    pub fn contains_none(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| !self.cards.contains(c))
    }

    pub fn sorted(mut self) -> Self {
        self.cards.sort_by_key(|c| (c.rank(), c.suit()));
        self.cards.reverse();
        self
    }

    pub fn shuffled(mut self, rng: &mut impl Rng) -> Self {
        self.cards.shuffle(rng);
        self
    }
}

impl<T> Default for Pile<T> {
    fn default() -> Self {
        Self {
            cards: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> From<&[Card]> for Pile<T> {
    fn from(value: &[Card]) -> Self {
        Self {
            cards: value.to_vec(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> From<Vec<Card>> for Pile<T> {
    fn from(value: Vec<Card>) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T> std::ops::Deref for Pile<T> {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        self.cards.as_slice()
    }
}

impl<T> std::fmt::Debug for Pile<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", crate::card::cards_to_string(&self.cards))
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests;
