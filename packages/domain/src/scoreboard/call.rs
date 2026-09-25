mod kind;

use kind::CallKind;

// ------------------------------------
use std::collections::HashSet;

use crate::{Card, Face, Points, Value, constants::PLAY_TARGET};

/// A single scoring call made during a cribbage hand.
///
/// A `Call` pairs a [`ScoreKind`] with the exact cards that justify the points.
/// Examples include “fifteen for two”, “a pair”, “a run of three”, “His Heels”,
/// or “one for his nobs”.
///
/// Multiple calls may be combined inside a larger scoring event (for example
/// when counting a hand that contains both pairs and runs).
#[derive(Clone, PartialEq, Eq)]
pub struct Call {
    /// The kind of score being claimed (fifteen, pair, run, etc.).
    kind: CallKind,

    /// The cards that form this scoring combination.
    ///
    /// The order of cards is not significant for most kinds, but may be useful
    /// for display or for reconstructing the play sequence.
    cards: Vec<Card>,
}

impl Call {
    fn new(kind: CallKind, cards: &[Card]) -> Self {
        Self {
            kind,
            cards: cards.to_vec(),
        }
    }

    /// Returns a scoring call if the cards total fifteen.
    pub fn try_fifteen(cards: &[Card]) -> Option<Call> {
        (cards.iter().map(|c| c.value()).sum::<Value>() == 15.into())
            .then(|| Self::new(CallKind::Fifteen, cards))
    }

    /// Returns a scoring call if the cards total fifteen.
    /// Panic on no score.
    #[cfg(test)]
    pub fn fifteen(cards: &[Card]) -> Call {
        Self::try_fifteen(cards).expect("require call fifteen")
    }

    /// Returns a scoring call if exactly two cards have the same face.
    pub fn try_pair(cards: &[Card]) -> Option<Call> {
        Self::same_n_faces(2, cards).then(|| Self::new(CallKind::Pair, cards))
    }

    /// Returns a scoring call if exactly two cards have the same face.
    /// Panic on no score.
    #[cfg(test)]
    pub fn pair(cards: &[Card]) -> Call {
        Self::try_pair(cards).expect("require call pair")
    }

    /// Returns a scoring call if exactly three cards have the same face.
    pub fn try_triplet(cards: &[Card]) -> Option<Call> {
        Self::same_n_faces(3, cards).then(|| Self::new(CallKind::Triplet, cards))
    }

    /// Returns a scoring call if exactly three cards have the same face.
    /// Panic on no score.
    #[cfg(test)]
    pub fn triplet(cards: &[Card]) -> Call {
        Self::try_triplet(cards).expect("require call triplet")
    }

    /// Returns a scoring call if exactly four cards have the same face.
    pub fn try_quadruplet(cards: &[Card]) -> Option<Call> {
        Self::same_n_faces(4, cards).then(|| Self::new(CallKind::Quadruplet, cards))
    }

    /// Returns a scoring call if exactly four cards have the same face.
    /// Panic on no score.
    #[cfg(test)]
    pub fn quadruplet(cards: &[Card]) -> Call {
        Self::try_quadruplet(cards).expect("require call quadruplet")
    }

    fn same_n_faces(n: usize, cards: &[Card]) -> bool {
        let n_cards = cards.len() == n;
        let same_face = cards.iter().map(Card::face).collect::<HashSet<_>>().len() == 1;
        n_cards && same_face
    }

    /// Returns a scoring call if the cards form a consecutive run.
    pub fn try_run(cards: &[Card]) -> Option<Call> {
        let mut cards = cards.to_vec();
        cards.sort_by_key(Card::rank);
        cards
            .windows(2)
            .map(|cs| cs[1].rank() - cs[0].rank())
            .all(|d| d == 1)
            .then(|| Self::new(CallKind::Run, cards.as_slice()))
    }

    /// Returns a scoring call if the cards form a consecutive run.
    /// Panic on no score.
    #[cfg(test)]
    pub fn run(cards: &[Card]) -> Call {
        Self::try_run(cards).expect("require call run")
    }

    /// Returns a scoring call if the cards form a flush.
    pub fn try_flush(cards: &[Card]) -> Option<Call> {
        (cards.iter().map(Card::suit).collect::<HashSet<_>>().len() == 1)
            .then(|| Self::new(CallKind::Flush, cards))
    }

    /// Returns a scoring call if the cards form a flush.
    /// Panic on no score.
    #[cfg(test)]
    pub fn flush(cards: &[Card]) -> Call {
        Self::try_flush(cards).expect("require call flush")
    }

    /// Returns a scoring call if the last card has been played.
    pub fn try_lastcard(played_cards: &[Card], remaining_cards: &[Card]) -> Option<Call> {
        let total = played_cards.iter().map(Card::value).sum::<Value>();
        let playable = remaining_cards
            .iter()
            .filter(|c| total + c.value() <= Value::from(PLAY_TARGET));
        (playable.count() == 0 && total != Value::from(PLAY_TARGET))
            .then(|| Self::new(CallKind::LastCard, played_cards))
    }

    /// Returns a scoring call if the last card has been played.
    /// Panic on no score.
    #[cfg(test)]
    pub fn lastcard(played_cards: &[Card], remaining_cards: &[Card]) -> Call {
        Self::try_lastcard(played_cards, remaining_cards).expect("require call lastcard")
    }

    /// Returns a scoring call if the running total is 31.
    pub fn try_thirtyone(played_cards: &[Card]) -> Option<Call> {
        let total = played_cards.iter().map(Card::value).sum::<Value>();
        (total == Value::from(PLAY_TARGET)).then(|| Self::new(CallKind::ThirtyOne, played_cards))
    }

    /// Returns a scoring call if the running total is 31.
    /// Panic on no score.
    #[cfg(test)]
    pub fn thirtyone(cards: &[Card]) -> Call {
        Self::try_thirtyone(cards).expect("require call thirtyone")
    }

    /// Returns a scoring call if the starter card is a Jack.
    pub fn try_hisheels(card: Card) -> Option<Call> {
        (card.face() == Face::Jack).then(|| Self::new(CallKind::HisHeels, [card].as_ref()))
    }

    /// Returns a scoring call if the starter card is a Jack.
    /// Panic on no score.
    #[cfg(test)]
    pub fn hisheels(card: Card) -> Call {
        Self::try_hisheels(card).expect("require call hisheels")
    }

    /// Returns a scoring call the card is a Jack and same suit as the cut card.
    pub fn try_nobs(card: Card, cut: Card) -> Option<Call> {
        (card.face() == Face::Jack && card.suit() == cut.suit())
            .then(|| Self::new(CallKind::Nobs, [card, cut].as_ref()))
    }

    /// Returns a scoring call the card is a Jack and same suit as the cut card.
    /// Panic on no score.
    #[cfg(test)]
    pub fn nobs(card: Card, cut: Card) -> Call {
        Self::try_nobs(card, cut).expect("require call nobs")
    }

    /// Return the points awarded for this kind of call.
    pub fn points(&self) -> Points {
        match self.kind {
            CallKind::Fifteen => 2.into(),
            CallKind::Pair => 2.into(),
            CallKind::Triplet => 6.into(),
            CallKind::Quadruplet => 12.into(),
            CallKind::Run => self.cards.len().into(),
            CallKind::Flush => self.cards.len().into(),
            CallKind::LastCard => 1.into(),
            CallKind::ThirtyOne => 2.into(),
            CallKind::HisHeels => 2.into(),
            CallKind::Nobs => 1.into(),
        }
    }
}

impl std::fmt::Debug for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}({:?})",
            self.kind,
            crate::card::cards_to_string(&self.cards)
        )
    }
}

pub fn calls_to_string(calls: &[Call]) -> String {
    calls
        .iter()
        .map(|c| format!("{:?}", c))
        .collect::<Vec<_>>()
        .join(", ")
}
