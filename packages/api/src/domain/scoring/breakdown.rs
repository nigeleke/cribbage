use itertools::*;
use serde::{Deserialize, Serialize};

use super::constants::*;
use crate::constants::*;
use crate::display::format_vec;
use crate::{Card, Crib, Cut, Hand, PlayState, Points, ScoreEvent, ScoreKind, Value};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Breakdown(Vec<ScoreEvent>);

impl Breakdown {
    fn add_event(&mut self, kind: ScoreKind, cards: &[Card], points: Points) {
        let event = ScoreEvent::new(kind, Vec::from(cards), points);
        self.0.push(event);
    }

    pub fn points(&self) -> Points {
        self.0.iter().map(ScoreEvent::points).sum()
    }

    pub fn his_heels(cut: Card) -> Self {
        let mut breakdown = Self::default();

        if cut.is_jack() {
            breakdown.add_event(ScoreKind::HisHeels, &[cut], Points::from(SCORE_HIS_HEELS));
        }

        breakdown
    }

    pub fn play_card(play_state: &PlayState) -> Self {
        Self::default()
            .play_card_fifteens(play_state)
            .play_card_pairs(play_state)
            .play_card_runs(play_state)
            .play_card_last(play_state)
    }

    fn play_card_fifteens(mut self, play_state: &PlayState) -> Self {
        let cards = play_state
            .current_plays()
            .into_iter()
            .map(|p| p.card())
            .collect::<Vec<_>>();

        if play_state.running_total() == Value::from(15) {
            self.add_event(
                ScoreKind::Fifteen,
                cards.as_slice(),
                Points::from(SCORE_FIFTEEN),
            )
        }

        self
    }

    fn play_card_pairs(mut self, play_state: &PlayState) -> Self {
        let mut cards = play_state
            .current_plays()
            .into_iter()
            .map(|p| p.card())
            .rev();

        let first = cards.next().expect("cards.next");
        let matching = cards.take_while(|c| c.face() == first.face());

        let mut cards = Vec::from_iter(matching);
        cards.push(first);

        if let Some((kind, points)) = match cards.len() {
            1 => None,
            2 => Some((ScoreKind::Pair, SCORE_PAIR.into())),
            3 => Some((ScoreKind::Triplet, SCORE_ROYAL_PAIR.into())),
            4 => Some((ScoreKind::Quadruplet, SCORE_DOUBLE_ROYAL_PAIR.into())),
            _ => unreachable!("never >4 cards with same face"),
        } {
            self.add_event(kind, cards.as_slice(), points);
        }

        self
    }

    fn play_card_runs(mut self, play_state: &PlayState) -> Self {
        let current_plays = &play_state.current_plays();

        for len in (MINIMUM_RUN_LENGTH..=current_plays.len()).rev() {
            let current_plays = current_plays.iter().rev();
            let mut cards = current_plays
                .map(|p| p.card())
                .take(len)
                .collect::<Vec<_>>();

            cards.sort_by(|&a, &b| a.rank().cmp(&b.rank()));

            let differences = cards
                .windows(2)
                .map(|w| w[1].rank() - w[0].rank())
                .collect::<Vec<_>>();

            let sequential = differences.iter().all(|d| *d == 1);
            if sequential {
                self.add_event(ScoreKind::Run, cards.as_slice(), Points::from(len));
                break;
            }
        }

        self
    }

    fn play_card_last(mut self, play_state: &PlayState) -> Self {
        if play_state.is_current_play_finished() {
            let cards = play_state
                .current_plays()
                .iter()
                .map(|p| p.card())
                .collect::<Vec<_>>();

            if play_state.running_total() == Value::from(PLAY_TARGET) {
                self.add_event(
                    ScoreKind::ThirtyOne,
                    cards.as_slice(),
                    Points::from(SCORE_THIRTY_ONE),
                );
            } else {
                self.add_event(ScoreKind::Go, &[], Points::from(SCORE_UNDER_THIRTY_ONE));
            }
        }

        self
    }

    pub fn pass(play_state: &PlayState) -> Self {
        Self::default().pass_last_card(play_state)
    }

    fn pass_last_card(mut self, play_state: &PlayState) -> Self {
        if play_state.is_current_play_finished() && play_state.all_players_passed() {
            self.add_event(ScoreKind::Go, &[], Points::from(SCORE_UNDER_THIRTY_ONE))
        }

        self
    }

    pub fn hand(hand: &Hand, cut: Cut) -> Self {
        let mut all = hand.clone();
        all.add(cut);

        Self::default()
            .fifteens(all.as_ref())
            .pairs(all.as_ref())
            .runs(all.as_ref())
            .flush(hand.as_ref(), cut, 4)
            .nobs(hand.as_ref(), cut)
    }

    pub fn crib(hand: &Crib, cut: Cut) -> Self {
        let mut all = hand.clone();
        all.add(cut);

        Self::default()
            .fifteens(all.as_ref())
            .pairs(all.as_ref())
            .runs(all.as_ref())
            .flush(hand.as_ref(), cut, 5)
            .nobs(hand.as_ref(), cut)
    }

    fn fifteens(mut self, cards: &[Card]) -> Self {
        for n in 2..=cards.len() {
            for combination in cards.as_ref().iter().combinations(n) {
                let combination_total: Value = combination.iter().map(|c| c.value()).sum();

                if combination_total == 15.into() {
                    let cards = combination.iter().map(|c| **c).collect::<Vec<_>>();
                    self.add_event(ScoreKind::Fifteen, &cards, Points::from(SCORE_FIFTEEN));
                }
            }
        }

        self
    }

    fn pairs(mut self, cards: &[Card]) -> Self {
        for combination in cards.as_ref().iter().combinations(2) {
            let mut combination = combination.into_iter();
            let (one, two) = (combination.next().unwrap(), combination.next().unwrap());
            if one.face() == two.face() {
                self.add_event(ScoreKind::Pair, &[*one, *two], Points::from(SCORE_PAIR));
            }
        }

        self
    }

    fn runs(mut self, cards: &[Card]) -> Self {
        let mut cards = Vec::from(cards);
        cards.sort_by(|c1, c2| c1.rank().cmp(&c2.rank()));

        for len in (MINIMUM_RUN_LENGTH..=cards.len()).rev() {
            let mut points = Points::default();

            for combination in cards.iter().combinations(len) {
                let differences = combination
                    .windows(2)
                    .map(|cs| cs[1].rank() - cs[0].rank())
                    .collect::<Vec<_>>();

                let sequential = differences.iter().all(|d| *d == 1);
                if sequential {
                    let combination = combination.into_iter().cloned().collect::<Vec<_>>();
                    points = Points::from(combination.len());
                    self.add_event(ScoreKind::Run, &combination, points);
                }
            }

            if points != Points::default() {
                break;
            }
        }

        self
    }

    fn flush(mut self, cards: &[Card], cut: Cut, constaint: usize) -> Self {
        let flush = |cards: &[Card]| {
            let suit = cards.first().map(|c| c.suit()).unwrap();
            let same_suit = cards.iter().all(|c| c.suit() == suit);
            if same_suit {
                Points::from(cards.len())
            } else {
                Points::default()
            }
        };

        let mut all = Vec::from(cards);
        all.push(cut);

        let flush_all = flush(&all);
        let flush_cards = flush(cards);

        if flush_all >= Points::from(constaint) {
            self.add_event(ScoreKind::Flush, &all, flush_all);
        } else if flush_cards >= Points::from(constaint) {
            self.add_event(ScoreKind::Fifteen, cards, flush_cards);
        }

        self
    }

    fn nobs(mut self, cards: &[Card], cut: Cut) -> Self {
        for card in cards {
            if card.is_jack() && card.suit() == cut.suit() {
                self.add_event(ScoreKind::Nobs, cards, Points::from(SCORE_NOBS));
            }
        }

        self
    }
}

impl std::fmt::Display for Breakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let events = format_vec(self.0.as_slice());
        events.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use crate::{Card, Hand, PLAYER0, PLAYER1, card, hand};

    #[test]
    #[should_panic]
    fn impossible_pairs_will_panic() {
        let hand1 = hand!("AHACADASAH");
        let hand2 = hand!("");

        let mut play_state = PlayState::new(PLAYER0)
            .with_pending_plays(PLAYER0, &hand1.as_ref())
            .with_pending_plays(PLAYER1, &hand2.as_ref());
        play_state.play(card!("AH"));
        play_state.play(card!("AC"));
        play_state.play(card!("AD"));
        play_state.play(card!("AS"));
        play_state.play(card!("AH"));

        let _ = Breakdown::play_card(&play_state).points();
    }
}
