use super::constants::*;
use crate::{
    Card, HasFace, HasRank, PlayState, Points, ScoreEvent, ScoreKind, Value, constants::*,
};
use serde::{Deserialize, Serialize};

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

        if cut.face().is_jack() {
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
                self.add_event(
                    ScoreKind::Go,
                    cards.as_slice(),
                    Points::from(SCORE_UNDER_THIRTY_ONE),
                );
            }
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Card, Hand, PLAYER0, PLAYER1, card, hand};
    use std::str::FromStr;

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
