use crate::{
    Card, Hand, Hands, HasValue, PLAYER0, PLAYER1, PLAYERS, Play, Player, Value, constants::*,
    display::format_vec,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayState {
    next_to_play: Player,
    pending_plays: Vec<Play>,
    pass_count: usize,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {
    pub fn new(next_to_play: Player) -> Self {
        Self {
            next_to_play,
            pending_plays: Vec::default(),
            pass_count: 0,
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
        }
    }

    pub fn with_pending_plays(mut self, player: Player, cards: &[Card]) -> Self {
        let mut plays = cards
            .iter()
            .map(|card| Play::new(player, *card))
            .collect::<Vec<_>>();
        self.pending_plays.append(&mut plays);
        self
    }

    pub fn running_total(&self) -> Value {
        self.current_plays.iter().map(|p| p.card().value()).sum()
    }

    pub fn legal_plays(&self, player: Player) -> Vec<Card> {
        let is_player = |p: &Play| p.player() == player;
        let running_total = self.running_total();
        let is_in_limit = |p: &Play| running_total + p.card().value() <= Value::from(PLAY_TARGET);
        let playable = |p: &Play| is_player(p) && is_in_limit(p);

        self.pending_plays
            .iter()
            .filter_map(|p| playable(p).then_some(p.card()))
            .collect()
    }

    pub const fn pass_count(&self) -> usize {
        self.pass_count
    }

    pub fn current_plays(&self) -> Vec<Play> {
        self.current_plays.clone()
    }

    pub fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }

    pub fn play(&mut self, card: Card) {
        let player = self.next_to_play;
        if self.pass_count() == 0 {
            self.make_opponent_next_player();
        }

        self.pending_plays
            .retain(|p| p.player() != player || p.card() != card);
        self.current_plays.push(Play::new(player, card));
    }

    pub const fn pass(&mut self) {
        self.make_opponent_next_player();
        self.pass_count += 1;
    }

    const fn make_opponent_next_player(&mut self) {
        self.next_to_play = self.next_to_play.opponent();
    }

    pub fn is_current_play_finished(&self) -> bool {
        let running_total = self.running_total();
        let pending_plays = &self.pending_plays;
        pending_plays
            .iter()
            .all(|play| play.card().value() + running_total > Value::from(PLAY_TARGET))
    }

    pub fn start_new_play(&mut self) {
        self.previous_plays.append(&mut self.current_plays);
        self.pass_count = 0;
    }

    pub fn target_reached(&self) -> bool {
        self.running_total() == Value::from(PLAY_TARGET)
    }

    pub const fn all_cards_are_played(&self) -> bool {
        self.pending_plays.is_empty()
    }

    pub fn finish_plays(&mut self) -> Hands {
        let hands = self.regather_hands();
        self.current_plays = Vec::default();
        self.previous_plays = Vec::default();
        hands
    }

    pub const fn next_to_play(&self) -> Player {
        self.next_to_play
    }

    fn regather_hands(&self) -> Hands {
        let mut previous_plays = self.previous_plays();
        let mut plays = self.current_plays();
        plays.append(&mut previous_plays);

        let hands = PLAYERS
            .into_iter()
            .map(|player| {
                plays
                    .iter()
                    .filter_map(|p| (p.player() == player).then_some(p.card()))
                    .collect::<Hand>()
            })
            .collect::<Vec<_>>();
        let hands = hands.as_slice();

        [hands[PLAYER0].clone(), hands[PLAYER1].clone()]
    }

    #[cfg(test)]
    pub fn force_current_play(&mut self, player: Player, card: Card) {
        self.current_plays.push(Play::new(player, card))
    }

    #[cfg(test)]
    pub fn force_previous_play(&mut self, player: Player, card: Card) {
        self.previous_plays.push(Play::new(player, card))
    }

    #[cfg(test)]
    pub const fn force_pass_count(&mut self, n: usize) {
        self.pass_count = n;
    }
}

impl std::fmt::Display for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Next({}), Legal(), Passes({}), Current({}), Previous({})",
            self.next_to_play,
            // format_hashmap(&self.legal_plays),
            self.pass_count,
            format_vec(&self.current_plays),
            format_vec(&self.previous_plays)
        )
    }
}
