mod flush_rule;

use flush_rule::FlushRule;

// ------------------------------------
use itertools::Itertools;

use crate::constants::*;
use crate::game::{GoStatus, PlayState};
use crate::{Call, Card, Crib, Hand, Player, Points};

/// One atomic scoring action.
#[derive(Clone, PartialEq, Eq)]
pub struct Event {
    player: Player,
    calls: Vec<Call>,
}

impl Event {
    /// Returns scoring events for a starter card.
    pub fn try_starter(player: Player, starter: Card) -> Option<Self> {
        Call::try_hisheels(starter).map(|call| Self::new(player, &[call]))
    }

    /// Returns scoring events for a play.
    pub fn try_play(player: Player, play: &PlayState) -> Option<Self> {
        let calls = Self::play_fifteen_calls(play)
            .chain(Self::play_pairs(play))
            .chain(Self::play_runs(play))
            .chain(Self::play_31(play))
            .chain(Self::play_last_card(play))
            .collect::<Vec<_>>();

        (!calls.is_empty()).then(|| Self::new(player, &calls))
    }

    fn play_fifteen_calls(play: &PlayState) -> impl Iterator<Item = Call> {
        let played_cards = &play.played_cards();
        Call::try_fifteen(played_cards).into_iter()
    }

    fn play_pairs(play: &PlayState) -> impl Iterator<Item = Call> {
        let cards = play
            .played_cards()
            .iter()
            .rev()
            .copied()
            .collect::<Vec<_>>();

        [
            cards.get(..4).and_then(Call::try_quadruplet),
            cards.get(..3).and_then(Call::try_triplet),
            cards.get(..2).and_then(Call::try_pair),
        ]
        .into_iter()
        .flatten()
        .take(1)
    }

    fn play_runs(play: &PlayState) -> impl Iterator<Item = Call> {
        let cards = play
            .played_cards()
            .iter()
            .rev()
            .copied()
            .collect::<Vec<_>>();
        (MINIMUM_RUN_LENGTH..=cards.len())
            .rev()
            .find_map(|len| cards.get(..len).and_then(Call::try_run))
            .into_iter()
    }

    fn play_31(play: &PlayState) -> impl Iterator<Item = Call> {
        Call::try_thirtyone(&play.played_cards()).into_iter()
    }

    fn play_last_card(play: &PlayState) -> impl Iterator<Item = Call> {
        let played_cards = play.played_cards();
        let remaining_cards = play.all_legal_plays();
        Call::try_lastcard(&played_cards, &remaining_cards).into_iter()
    }

    /// Returns scoring events for a go.
    pub fn try_go(player: Player, play: &PlayState) -> Option<Self> {
        let calls = Self::go_last_card(play).collect::<Vec<_>>();
        (!calls.is_empty()).then(|| Self::new(player, &calls))
    }

    fn go_last_card(play: &PlayState) -> impl Iterator<Item = Call> {
        let played_cards = &play.played_cards();
        let remaining_cards = &play.all_legal_plays();
        (!matches!(play.go_status(), GoStatus::NotCalled))
            .then(|| Call::try_lastcard(played_cards, remaining_cards))
            .into_iter()
            .flatten()
            .into_iter()
    }

    /// Returns scoring events for a pone hand.
    pub fn try_pone_hand(player: Player, hand: &Hand, cut: Card) -> Option<Self> {
        Self::try_cards(player, hand, cut, FlushRule::Hand)
    }

    /// Returns scoring events for a dealer hand.
    pub fn try_dealer_hand(player: Player, hand: &Hand, cut: Card) -> Option<Self> {
        Self::try_cards(player, hand, cut, FlushRule::Hand)
    }

    /// Returns scoring events for the crib.
    pub fn try_crib(player: Player, crib: &Crib, cut: Card) -> Option<Self> {
        Self::try_cards(player, crib, cut, FlushRule::Crib)
    }

    fn try_cards(player: Player, cards: &[Card], cut: Card, flush_rule: FlushRule) -> Option<Self> {
        let mut all_cards = cards.to_vec();
        all_cards.push(cut);

        let calls = Self::cards_fifteens(&all_cards)
            .chain(Self::cards_pairs(&all_cards))
            .chain(Self::cards_runs(&all_cards))
            .chain(Self::cards_flush(cards, cut, flush_rule))
            .chain(Self::cards_nobs(cards, cut))
            .collect::<Vec<_>>();

        (!calls.is_empty()).then(|| Self::new(player, &calls))
    }

    fn cards_fifteens(cards: &[Card]) -> impl Iterator<Item = Call> {
        (2..=cards.len())
            .flat_map(|n| cards.iter().copied().combinations(n))
            .filter_map(|cards| Call::try_fifteen(&cards))
    }

    fn cards_pairs(cards: &[Card]) -> impl Iterator<Item = Call> {
        cards
            .iter()
            .copied()
            .combinations(2)
            .filter_map(|cards| Call::try_pair(&cards))
    }

    fn cards_flush(cards: &[Card], cut: Card, flush_rule: FlushRule) -> impl Iterator<Item = Call> {
        let mut all_cards = cards.to_vec();
        all_cards.push(cut);

        match flush_rule {
            FlushRule::Hand => Call::try_flush(&all_cards).or_else(|| Call::try_flush(cards)),
            FlushRule::Crib => Call::try_flush(&all_cards),
        }
        .into_iter()
    }

    fn cards_runs(cards: &[Card]) -> impl Iterator<Item = Call> {
        let runs_of_length = |n| {
            let mut runs = cards
                .iter()
                .copied()
                .combinations(n)
                .filter_map(|cards| Call::try_run(&cards));

            runs.next().map(|first| std::iter::once(first).chain(runs))
        };

        (MINIMUM_RUN_LENGTH..=cards.len())
            .rev()
            .find_map(runs_of_length)
            .into_iter()
            .flatten()
    }

    fn cards_nobs(cards: &[Card], cut: Card) -> impl Iterator<Item = Call> {
        cards
            .iter()
            .filter_map(move |card| Call::try_nobs(*card, cut))
            .into_iter()
    }

    /// Wrap card scoring calls into a single event to score for a player.
    pub fn new(player: Player, calls: &[Call]) -> Self {
        Self {
            player,
            calls: calls.to_vec(),
        }
    }

    /// Return the player who won the points for this event.
    pub fn player(&self) -> Player {
        self.player
    }

    /// Return total points from all of the calls.
    pub fn points(&self) -> Points {
        self.calls.iter().map(|c| c.points()).sum()
    }
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} scored {:?} <== {}",
            self.player,
            self.points(),
            crate::scoreboard::call::calls_to_string(&self.calls)
        )
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests;
