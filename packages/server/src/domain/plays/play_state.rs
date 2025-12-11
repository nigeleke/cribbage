use serde::{Deserialize, Serialize};
use tracing::field::debug;

use crate::{
    display::format_vec,
    domain::{
        Card, GoStatus, Hand, Hands, PLAYER0, PLAYER1, PLAYERS, Play, Player, ScoreSheet, Value,
        constants::PLAY_TARGET,
    },
};

/// Represents the current state of play during the pegging phase.
///
/// Tracks which player's turn it is, pending cards, the go status,
/// current and previous plays.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayState {
    next_to_play: Player,
    pending_plays: Vec<Vec<Card>>,
    go_status: GoStatus,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

/// Trait for types that expose a `PlayState`.
pub trait HasPlayState {
    /// Returns an immutable reference to the play state.
    fn play_state(&self) -> &PlayState;

    /// Returns a mutable reference to the play state.
    fn play_state_mut(&mut self) -> &mut PlayState;
}

impl PlayState {
    /// Creates a new `PlayState` with the specified next player to play.
    pub fn new(next_to_play: Player) -> Self {
        Self {
            next_to_play,
            pending_plays: vec![Vec::default(), Vec::default()],
            go_status: GoStatus::default(),
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
        }
    }

    /// Returns the player whose turn is next.
    #[must_use]
    pub const fn next_to_play(&self) -> Player {
        self.next_to_play
    }

    /// Sets pending plays for a player and returns the updated state.
    ///
    /// (Note: This is really part of the constructor information)
    #[must_use]
    pub fn with_pending_plays(mut self, player: Player, cards: &[Card]) -> Self {
        self.pending_plays.as_mut_slice()[player] = Vec::from(cards);
        self
    }

    /// Returns the current running total of points in the play sequence.
    #[must_use]
    pub fn running_total(&self) -> Value {
        self.current_plays.iter().map(|p| p.card().value()).sum()
    }

    /// Returns `true` if the specified player has any cards left to play regardless
    /// whether they are currently legal plays or not.
    #[must_use]
    pub fn has_cards(&self, player: Player) -> bool {
        !self.pending_plays[player].is_empty()
    }

    /// Returns the legal cards the specified player may play without exceeding the play limit.
    #[must_use]
    pub fn legal_plays(&self, player: Player) -> Vec<Card> {
        let running_total = self.running_total();
        let is_in_limit = |c: &Card| running_total + c.value() <= Value::from(PLAY_TARGET);

        self.pending_plays[player]
            .iter()
            .filter_map(|c| is_in_limit(c).then_some(*c))
            .collect::<Vec<_>>()
    }

    /// Returns an immutable reference to the current plays.
    #[must_use]
    pub fn current_plays(&self) -> &Vec<Play> {
        &self.current_plays
    }

    /// Returns an immutable reference to the previous plays.
    #[must_use]
    pub fn previous_plays(&self) -> &Vec<Play> {
        &self.previous_plays
    }

    /// Plays a card for the current player, updating the state and returning the resulting
    /// score sheet for the played card.
    #[must_use]
    pub fn play(&mut self, card: Card) -> ScoreSheet {
        let player = self.next_to_play;
        let opponent = player.opponent();

        self.pending_plays.as_mut_slice()[player].retain(|c| c != &card);
        self.current_plays.push(Play::new(player, card));
        let sheet = ScoreSheet::play_card(self);

        let reached_target = self.running_total() == Value::from(PLAY_TARGET);
        let player_has_cards = self.has_cards(player);
        let opponent_has_cards = self.has_cards(opponent);

        if reached_target {
            self.start_new_play();
        } else {
            match self.go_status {
                GoStatus::NotCalled => {
                    if opponent_has_cards {
                        self.next_to_play = opponent;
                    } else {
                        self.go_status = GoStatus::Called;
                    }
                }
                GoStatus::Called => {
                    self.go_status = GoStatus::PlayContinued;
                    // next_to_play remains player until they run out of cards
                    if !player_has_cards {
                        self.start_new_play();
                    }
                }
                GoStatus::PlayContinued => {
                    // next_to_play remains player until they run out of cards
                    if !player_has_cards {
                        self.start_new_play();
                    }
                }
            }
        }

        sheet
    }

    /// Calls "go" for the current player, updating the state and returning the resulting score sheet.
    #[must_use]
    pub fn go(&mut self) -> ScoreSheet {
        let player = self.next_to_play;
        let opponent = player.opponent();

        let sheet = ScoreSheet::go(self);

        let opponent_has_cards = self.has_cards(opponent);

        match self.go_status {
            GoStatus::NotCalled => {
                self.go_status = GoStatus::Called;

                if opponent_has_cards {
                    self.next_to_play = opponent;
                } else {
                    self.start_new_play();
                }
            }
            GoStatus::Called | GoStatus::PlayContinued => {
                self.start_new_play();
            }
        }

        sheet
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
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.pending_plays.iter().all(Vec::is_empty)
    }

    /// Finishes the current plays and returns the regathered hands.
    #[must_use]
    pub fn finish_plays(&mut self) -> Hands {
        let hands = self.regather_hands();
        self.current_plays = Vec::default();
        self.previous_plays = Vec::default();
        hands
    }

    fn regather_hands(&self) -> Hands {
        let plays = self
            .previous_plays
            .iter()
            .chain(self.current_plays.iter())
            .collect::<Vec<_>>();

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

    /// Returns the current go status.
    #[must_use]
    pub fn go_status(&self) -> &GoStatus {
        &self.go_status
    }

    #[cfg(test)]
    pub(crate) fn go_status_mut(&mut self) -> &mut GoStatus {
        &mut self.go_status
    }

    #[cfg(test)]
    pub(crate) fn current_plays_mut(&mut self) -> &mut Vec<Play> {
        &mut self.current_plays
    }

    #[cfg(test)]
    pub(crate) fn previous_plays_mut(&mut self) -> &mut Vec<Play> {
        &mut self.previous_plays
    }
}

impl std::fmt::Display for PlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Next({}), GoStatus({}), Pending({} -> {}, {} -> {}), Current({}), Previous({})",
            self.next_to_play,
            self.go_status.as_ref(),
            PLAYER0,
            format_vec(&self.pending_plays[PLAYER0]),
            PLAYER1,
            format_vec(&self.pending_plays[PLAYER1]),
            format_vec(&self.current_plays),
            format_vec(&self.previous_plays)
        )
    }
}
