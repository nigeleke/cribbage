use crate::constants::*;
use crate::{Card, Hand, Hands, PLAYERS, Player, Players, ScoreEvent, Value};

use super::{GoStatus, Play};

/// Represents the current state of play during the pegging phase.
///
/// Tracks which player's turn it is, pending cards, the go status,
/// current and previous plays.
#[derive(Clone, PartialEq, Eq)]
pub struct PlayState {
    next_to_play: Player,
    pending_plays: Players<Vec<Card>>,
    go_status: GoStatus,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {
    /// Creates a new `PlayState` with the specified next player to play.
    pub fn new(next_to_play: Player, hands: &Hands) -> Self {
        let initial = Self {
            next_to_play,
            pending_plays: [Vec::default(), Vec::default()].into(),
            go_status: GoStatus::default(),
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
        };

        PLAYERS.into_iter().fold(initial, |mut acc, player| {
            acc.pending_plays[player] = hands[player].to_vec();
            acc
        })
    }

    /// Add GoStatus for testing...
    #[cfg(test)]
    pub fn with_go_status(mut self, status: GoStatus) -> Self {
        self.go_status = status;
        self
    }

    /// Add current_plays for testing...
    #[cfg(test)]
    pub fn with_current_plays(mut self, plays: &[Play]) -> Self {
        self.current_plays = plays.to_vec();
        self
    }

    /// Add previous_plays for testing...
    #[cfg(test)]
    pub fn with_previous_plays(mut self, plays: &[Play]) -> Self {
        self.previous_plays = plays.to_vec();
        self
    }

    /// Returns the player whose turn is next.
    pub const fn next_to_play(&self) -> Player {
        self.next_to_play
    }

    /// Returns the cards that have been played in the current play phase.
    pub fn played_cards(&self) -> Vec<Card> {
        self.current_plays.iter().map(Play::card).collect()
    }

    /// Returns the current running total of points in the play sequence.
    pub fn running_total(&self) -> Value {
        self.current_plays.iter().map(|p| p.card().value()).sum()
    }

    /// Returns the cards that can still be legally played by either player.
    pub fn all_legal_plays(&self) -> Vec<Card> {
        PLAYERS
            .iter()
            .flat_map(|player| self.legal_plays(*player))
            .collect()
    }

    /// Returns the legal cards the specified player may play without exceeding the play limit.
    pub fn legal_plays(&self, player: Player) -> Vec<Card> {
        let running_total = self.running_total();
        let is_in_limit = |c: &Card| running_total + c.value() <= Value::from(PLAY_TARGET);

        self.pending_plays[player]
            .iter()
            .filter_map(|c| is_in_limit(c).then_some(*c))
            .collect::<Vec<_>>()
    }

    /// Plays a card for the current player, updating the state and returning any resulting
    /// scoring event for the played card.
    pub fn play(&mut self, card: Card) -> Option<ScoreEvent> {
        let player = self.next_to_play;
        let opponent = player.opponent();

        self.pending_plays[player].retain(|c| c != &card);
        self.current_plays.push(Play::new(player, card));
        let event = ScoreEvent::try_play(player, self);

        let reached_target = self.running_total() == Value::from(PLAY_TARGET);
        let player_has_cards = self.has_cards(player);
        let opponent_has_cards = self.has_cards(opponent);

        if reached_target {
            self.start_new_play();
        } else {
            match self.go_status {
                GoStatus::NotCalled if opponent_has_cards => self.next_to_play = opponent,
                GoStatus::NotCalled => {}
                GoStatus::Called | GoStatus::PlayContinued => {
                    self.go_status = GoStatus::PlayContinued;
                    // next_to_play remains player until they run out of cards
                    if !player_has_cards {
                        self.start_new_play();
                    }
                }
            }
        }

        event
    }

    fn has_cards(&self, player: Player) -> bool {
        !self.pending_plays[player].is_empty()
    }

    /// Calls "go" for the current player, updating the state and returning the resulting
    /// scoring event.
    pub fn go(&mut self) -> Option<ScoreEvent> {
        let player = self.next_to_play;
        let opponent = player.opponent();

        let opponent_has_cards = self.has_cards(opponent);

        let mut event = None;

        match self.go_status {
            GoStatus::NotCalled if opponent_has_cards => {
                self.go_status = GoStatus::Called;
                self.next_to_play = opponent;
            }
            GoStatus::NotCalled => {
                self.go_status = GoStatus::Called;
                event = ScoreEvent::try_go(player, self);
                self.start_new_play();
            }
            GoStatus::Called | GoStatus::PlayContinued => {
                event = ScoreEvent::try_go(player, self);
                self.start_new_play();
            }
        }

        event
    }

    fn start_new_play(&mut self) {
        // There will always be a valid play before a go can occur. The `or` condition
        // in `map_or` will never occur.
        let last_player = self
            .current_plays
            .last()
            .map_or(self.next_to_play, Play::player);
        let opponent = last_player.opponent();
        let opponent_has_cards = self.has_cards(opponent);

        self.next_to_play = if opponent_has_cards {
            opponent
        } else {
            last_player
        };

        self.previous_plays.append(&mut self.current_plays);

        self.go_status = GoStatus::NotCalled;
    }

    /// Returns `true` if all players have no cards left.
    pub fn is_finished(&self) -> bool {
        self.pending_plays.iter().all(Vec::is_empty)
    }

    /// Finishes the current plays and returns the regathered hands.
    pub fn finish_plays(&mut self) -> Hands {
        let hands = self.regather_hands();
        self.current_plays = Vec::default();
        self.previous_plays = Vec::default();
        hands
    }

    fn regather_hands(&self) -> Hands {
        Hands::from(std::array::from_fn(|player| {
            Hand::from(
                self.previous_plays
                    .iter()
                    .chain(self.current_plays.iter())
                    .filter_map(|play| (play.player().index() == player).then_some(play.card()))
                    .collect::<Vec<_>>(),
            )
        }))
    }

    /// Returns the current go status.
    pub fn go_status(&self) -> &GoStatus {
        &self.go_status
    }
}

impl std::fmt::Debug for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "play_state: (next: {:?}, go: {:?}, current: {:?}, previous: {:?})",
            self.next_to_play,
            self.go_status,
            super::play::plays_to_string(&self.current_plays),
            super::play::plays_to_string(&self.previous_plays)
        )
    }
}
