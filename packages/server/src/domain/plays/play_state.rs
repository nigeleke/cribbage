use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{
        Card, GoStatus, Hand, Hands, PLAYER0, PLAYER1, PLAYERS, Play, Player, ScoreSheet, Value,
        constants::PLAY_TARGET,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayState {
    next_to_play: Player,
    pending_plays: Vec<Vec<Card>>,
    go_status: GoStatus,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

pub trait HasPlayState {
    fn play_state(&self) -> &PlayState;
    fn play_state_mut(&mut self) -> &mut PlayState;
}

impl PlayState {
    pub fn new(next_to_play: Player) -> Self {
        Self {
            next_to_play,
            pending_plays: vec![Vec::default(), Vec::default()],
            go_status: GoStatus::default(),
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
        }
    }

    pub const fn next_to_play(&self) -> Player {
        self.next_to_play
    }

    pub fn with_pending_plays(mut self, player: Player, cards: &[Card]) -> Self {
        self.pending_plays.as_mut_slice()[player] = Vec::from(cards);
        self
    }

    pub fn has_cards(&self, player: Player) -> bool {
        !self.pending_plays[player].is_empty()
    }

    pub fn running_total(&self) -> Value {
        self.current_plays.iter().map(|p| p.card().value()).sum()
    }

    pub fn legal_plays(&self, player: Player) -> Vec<Card> {
        let running_total = self.running_total();
        let is_in_limit = |c: &Card| running_total + c.value() <= Value::from(PLAY_TARGET);

        self.pending_plays[player]
            .iter()
            .filter_map(|c| is_in_limit(c).then_some(*c))
            .collect::<Vec<_>>()
    }

    pub fn current_plays(&self) -> Vec<Play> {
        self.current_plays.clone()
    }

    pub fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }

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

    pub fn pass(&mut self) -> ScoreSheet {
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
        // There will always be a valid play before a pass can occur. The `or` condition
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

    pub fn is_finished(&self) -> bool {
        self.pending_plays.iter().all(Vec::is_empty)
    }

    pub fn finish_plays(&mut self) -> Hands {
        let hands = self.regather_hands();
        self.current_plays = Vec::default();
        self.previous_plays = Vec::default();
        hands
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

    pub fn go_status(&self) -> &GoStatus {
        &self.go_status
    }

    #[cfg(test)]
    pub fn go_status_mut(&mut self) -> &mut GoStatus {
        &mut self.go_status
    }

    #[cfg(test)]
    pub fn current_plays_mut(&mut self) -> &mut Vec<Play> {
        &mut self.current_plays
    }

    #[cfg(test)]
    pub fn previous_plays_mut(&mut self) -> &mut Vec<Play> {
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
