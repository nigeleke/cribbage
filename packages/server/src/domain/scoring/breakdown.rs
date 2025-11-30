use itertools::*;
use serde::{Deserialize, Serialize};

use super::constants::*;
use crate::{
    display::format_vec,
    domain::{
        Card, Crib, Hand, Play, PlayState, Points, ScoreEvent, ScoreKind, StarterCut, Value,
        constants::*,
    },
};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Breakdown(Vec<ScoreEvent>);

impl Breakdown {
    pub fn add_event(mut self, kind: ScoreKind, cards: &[Card], points: Points) -> Self {
        let event = ScoreEvent::new(kind, Vec::from(cards), points);
        self.0.push(event);
        self
    }

    pub fn add_event_if(
        mut self,
        condition: bool,
        kind: ScoreKind,
        cards: &[Card],
        points: Points,
    ) -> Self {
        if condition {
            let event = ScoreEvent::new(kind, Vec::from(cards), points);
            self.0.push(event);
        }
        self
    }

    pub fn points(&self) -> Points {
        self.0.iter().map(ScoreEvent::points).sum()
    }

    pub fn his_heels(cut: Card) -> Self {
        Self::default().add_event_if(
            cut.is_jack(),
            ScoreKind::HisHeels,
            &[cut],
            Points::from(SCORE_HIS_HEELS),
        )
    }

    pub fn play_card(play_state: &PlayState) -> Self {
        Self::default()
            .play_card_fifteens(play_state)
            .play_card_pairs(play_state)
            .play_card_runs(play_state)
            .play_card_last(play_state)
    }

    fn play_card_fifteens(self, play_state: &PlayState) -> Self {
        let cards = play_state
            .current_plays()
            .into_iter()
            .map(|p| p.card())
            .collect::<Vec<_>>();

        self.add_event_if(
            play_state.running_total() == Value::from(15),
            ScoreKind::Fifteen,
            cards.as_slice(),
            Points::from(SCORE_FIFTEEN),
        )
    }

    fn play_card_pairs(self, play_state: &PlayState) -> Self {
        let cards = play_state
            .current_plays()
            .iter()
            .rev()
            .map(Play::card)
            .collect::<Vec<_>>();

        let pair_info = cards.split_first().and_then(|(first, rest)| {
            let same_face = |card: &&Card| card.face() == first.face();
            let count = 1 + rest.iter().take_while(same_face).count();
            match count {
                2 => Some((ScoreKind::Pair, SCORE_PAIR)),
                3 => Some((ScoreKind::Triplet, SCORE_ROYAL_PAIR)),
                4 => Some((ScoreKind::Quadruplet, SCORE_DOUBLE_ROYAL_PAIR)),
                _ => None,
            }
            .map(|(kind, pts)| (kind, pts, count))
        });

        match pair_info {
            Some((kind, points, count)) => self.add_event(kind, &cards[..count], points.into()),
            None => self,
        }
    }

    fn play_card_runs(self, play_state: &PlayState) -> Self {
        let cards = play_state
            .current_plays()
            .iter()
            .rev()
            .map(Play::card)
            .collect::<Vec<_>>();

        let longest_run = (MINIMUM_RUN_LENGTH..=cards.len())
            .rev()
            .find(|&len| {
                let slice = &cards[..len];
                let mut ranks: Vec<_> = slice.iter().map(|c| c.rank()).collect();
                ranks.sort_unstable();
                ranks.windows(2).all(|w| w[1] == w[0] + 1)
            })
            .map(|len| {
                let mut run = cards[..len].to_vec();
                run.sort_by_key(|c| c.rank());
                (run, Points::from(len))
            });

        if let Some((cards, points)) = longest_run {
            self.add_event(ScoreKind::Run, cards.as_slice(), points)
        } else {
            self
        }
    }

    fn play_card_last(self, play_state: &PlayState) -> Self {
        let cards: Vec<_> = play_state
            .current_plays()
            .iter()
            .map(|p| p.card())
            .collect();

        let is_31 = play_state.running_total() == PLAY_TARGET.into();
        let is_finished = play_state.is_current_play_finished();

        if !is_finished {
            self
        } else if is_31 {
            self.add_event(ScoreKind::ThirtyOne, &cards, SCORE_THIRTY_ONE.into())
        } else {
            self.add_event(ScoreKind::Go, &[], SCORE_UNDER_THIRTY_ONE.into())
        }
    }

    pub fn pass(play_state: &PlayState) -> Self {
        Self::default().pass_last_card(play_state)
    }

    fn pass_last_card(self, play_state: &PlayState) -> Self {
        self.add_event_if(
            play_state.is_current_play_finished() && play_state.all_players_passed(),
            ScoreKind::Go,
            &[],
            Points::from(SCORE_UNDER_THIRTY_ONE),
        )
    }

    pub fn hand(hand: &Hand, cut: StarterCut) -> Self {
        let mut all = hand.clone();
        all.add(cut);

        Self::default()
            .fifteens(all.as_ref())
            .pairs(all.as_ref())
            .runs(all.as_ref())
            .flush(hand.as_ref(), cut, 4)
            .nobs(hand.as_ref(), cut)
    }

    pub fn crib(crib: &Crib, cut: StarterCut) -> Self {
        let mut all = crib.clone();
        all.add(cut);

        Self::default()
            .fifteens(all.as_ref())
            .pairs(all.as_ref())
            .runs(all.as_ref())
            .flush(crib.as_ref(), cut, 5)
            .nobs(crib.as_ref(), cut)
    }

    fn fifteens(self, cards: &[Card]) -> Self {
        (2..=cards.len())
            .flat_map(|n| cards.iter().combinations(n))
            .filter(|combo| combo.iter().map(|c| c.value()).sum::<Value>() == 15.into())
            .fold(self, |acc, combo| {
                let combo_cards: Vec<Card> = combo.iter().copied().copied().collect();
                acc.add_event(ScoreKind::Fifteen, &combo_cards, SCORE_FIFTEEN.into())
            })
    }

    fn pairs(self, cards: &[Card]) -> Self {
        cards
            .iter()
            .copied()
            .combinations(2)
            .filter(|pair| pair[0].face() == pair[1].face())
            .fold(self, |acc, pair| {
                acc.add_event(ScoreKind::Pair, &pair, SCORE_PAIR.into())
            })
    }

    fn runs(self, cards: &[Card]) -> Self {
        let mut scores = Vec::default();

        let mut cards = Vec::from(cards);
        cards.sort_by_key(|c| c.rank());

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
                    scores.push((combination, points))
                }
            }

            if points != Points::default() {
                break;
            }
        }

        scores
            .into_iter()
            .fold(self, |acc, e| acc.add_event(ScoreKind::Run, &e.0, e.1))
    }

    fn flush(self, cards: &[Card], cut: StarterCut, constraint: usize) -> Self {
        let mut all = Vec::from(cards);
        all.push(cut);

        let all_same_suit = |cs: &[Card]| cs.iter().map(|c| c.suit()).all_equal();

        let points_if_flush = |cs: &[Card]| {
            (cs.len() >= constraint)
                .then_some(all_same_suit(cs).then(|| Points::from(cs.len())))
                .flatten()
        };

        if let Some(points) = points_if_flush(&all) {
            self.add_event(ScoreKind::Flush, &all, points)
        } else if let Some(points) = points_if_flush(cards) {
            self.add_event(ScoreKind::Flush, cards, points)
        } else {
            self
        }
    }

    fn nobs(self, cards: &[Card], cut: StarterCut) -> Self {
        self.add_event_if(
            cards.iter().any(|c| c.is_jack() && c.suit() == cut.suit()),
            ScoreKind::Nobs,
            cards,
            SCORE_NOBS.into(),
        )
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
    use crate::{
        card,
        domain::{Card, Hand, PLAYER0, PLAYER1},
        hand,
    };

    #[test]
    fn impossible_pairs_will_return_0() {
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

        assert_eq!(
            Breakdown::play_card(&play_state).points(),
            Points::default()
        );
    }
}
