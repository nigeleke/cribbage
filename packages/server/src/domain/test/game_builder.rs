use std::str::FromStr;

use crate::{card, cards, crib, domain::*, hand};

#[derive(Debug)]
pub struct GameBuilder {
    dealer: usize,
    cuts: Vec<Card>,
    peggings: Vec<usize>,
    hands: Vec<Hand>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
    pass_count: usize,
    crib: Crib,
    cut: Option<Card>,
    deck: Deck,
    winner: usize,
    pending: Pending,
}

impl GameBuilder {
    pub fn with_cuts(mut self, cuts: &str) -> Self {
        let cuts = cards!(cuts);
        self.deck.remove_all(cuts.as_ref());
        cuts.iter().for_each(|c| self.cuts.push(*c));
        self
    }

    pub fn with_points(mut self, points0: usize, points1: usize) -> Self {
        self.peggings.push(points0);
        self.peggings.push(points1);
        self
    }

    pub fn with_hands(mut self, hand0: &str, hand1: &str) -> Self {
        let mut add_hand = |hand: &str| {
            let hand = hand!(hand);
            self.deck.remove_all(hand.as_ref());
            self.hands.push(hand);
        };

        add_hand(hand0);
        add_hand(hand1);

        self
    }

    pub fn with_crib(mut self, crib: &str) -> Self {
        let crib = crib!(crib);
        self.deck.remove_all(crib.as_ref());
        self.crib = crib;
        self
    }

    pub fn with_cut(mut self, cut: &str) -> Self {
        let cut = card!(cut);
        self.deck.remove(cut);
        self.cut = Some(cut);
        self
    }

    pub fn with_current_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays.into_iter().map(|(p, c)| (Player::from(*p), card!(c)));
        let plays = plays.map(|(p, c)| Play::new(p, c));
        self.current_plays = Vec::from_iter(plays);
        self
    }

    pub fn with_previous_plays(mut self, plays: &[(usize, &str)]) -> Self {
        let plays = plays.iter().map(|(p, c)| (Player::from(*p), card!(*c)));
        let plays = plays.map(|(p, c)| Play::new(p, c));
        self.previous_plays = Vec::from_iter(plays);
        self
    }

    pub fn with_pass(mut self) -> Self {
        self.pass_count += 1;
        self
    }

    pub fn with_winner(mut self, winner: usize) -> Self {
        self.winner = winner;
        self
    }

    pub fn with_ack(mut self, player: usize) -> Self {
        self.pending.acknowledge(Player::from(player));
        self
    }

    fn new_game(state: State) -> Game {
        let host = UserId::new();
        let guest = UserId::new();

        let mut game = Game::default();
        let game_id = GameId::new();

        let name = format!("test-game__{}", chrono::Utc::now());

        game.apply_event(GameEvent::ComputerGameStarted {
            game_id,
            host,
            guest,
            name,
        });
        *game.state_mut() = state;
        game
    }

    fn domain_cuts(&self) -> CutsForDeal {
        [Some(self.cuts[0]), Some(self.cuts[1])]
    }

    fn domain_scoreboard(&self) -> Scoreboard {
        let peggings = [
            Pegging::new(*self.peggings.get(0).unwrap_or(&0)),
            Pegging::new(*self.peggings.get(1).unwrap_or(&0)),
        ];
        Scoreboard::new(peggings)
    }

    #[inline]
    fn domain_roles(&self) -> Roles {
        Roles::new(Dealer::from(Player::from(self.dealer)))
    }

    #[inline]
    fn domain_hands(&self) -> Hands {
        [self.hands[0].clone(), self.hands[1].clone()]
    }

    #[inline]
    fn domain_crib(&self) -> Crib {
        self.crib.clone()
    }

    #[inline]
    fn domain_deck(&self) -> Deck {
        self.deck.clone()
    }

    fn domain_play_state(&self, next_to_play: usize) -> PlayState {
        let mut play_state = PlayState::new(Player::from(next_to_play))
            .with_pending_plays(PLAYER0, self.hands[0].as_ref())
            .with_pending_plays(PLAYER1, self.hands[1].as_ref());

        *play_state.pass_count_mut() = self.pass_count;
        *play_state.current_plays_mut() = self.current_plays.clone();
        *play_state.previous_plays_mut() = self.previous_plays.clone();

        play_state
    }

    #[inline]
    fn domain_cut(&self) -> Card {
        self.cut.expect("starter cut must be defined")
    }

    #[inline]
    fn domain_winner(&self) -> Player {
        Player::from(self.winner)
    }

    #[inline]
    fn domain_pending(&self) -> Pending {
        self.pending.clone()
    }

    pub fn into_starting(self) -> Game {
        let starting = Starting::new(
            self.domain_cuts(),
            self.domain_deck(),
            self.domain_pending(),
        );
        Self::new_game(State::Starting(starting))
    }

    pub fn into_discarding(mut self) -> Game {
        self.cut.into_iter().for_each(|c| self.deck.add(c));

        let discarding = Discarding::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_deck(),
            self.domain_pending(),
        );

        Self::new_game(State::Discarding(discarding))
    }

    pub fn into_playing(self, next_to_play: usize) -> Game {
        let playing = Playing::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_play_state(next_to_play),
            self.domain_crib(),
            self.domain_cut(),
            self.domain_pending(),
        );
        Self::new_game(State::Playing(playing))
    }

    pub fn into_scoring_pone(self) -> Game {
        let scores = ScoreBreakdown::hand(&self.hands[1 - self.dealer], self.domain_cut());
        let scoring = ScoringPone::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
            scores,
            self.domain_pending(),
        );
        Self::new_game(State::ScoringPone(scoring))
    }

    pub fn into_scoring_dealer(self) -> Game {
        let scores = ScoreBreakdown::hand(&self.hands[self.dealer], self.domain_cut());
        let scoring = ScoringDealer::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
            scores,
            self.domain_pending(),
        );
        Self::new_game(State::ScoringDealer(scoring))
    }

    pub fn into_scoring_crib(self) -> Game {
        let scores = ScoreBreakdown::crib(&self.crib, self.domain_cut());
        let scoring = ScoringCrib::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
            scores,
            self.domain_pending(),
        );
        Self::new_game(State::ScoringCrib(scoring))
    }

    pub fn into_finished(self) -> Game {
        let finished = Finished::new(
            self.domain_winner(),
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
        );
        Self::new_game(State::Finished(finished))
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            deck: Deck::shuffled_pack(),
            dealer: 0,
            cuts: Vec::default(),
            peggings: Vec::new(),
            hands: Vec::default(),
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
            pass_count: 0,
            crib: Crib::default(),
            cut: None,
            winner: 0,
            pending: Pending::default(),
        }
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! scenario {
    ( $final_method:ident ( $($final_arg:expr),* $(,)? ) ; $($method:ident( $($arg:expr),* $(,)? )),* $(,)? ) => {{
        let builder = GameBuilder::default()
            $( . $method ( $($arg),* ) )* ;

        let mut game = builder. $final_method ( $($final_arg),* );
        *game.name_mut() = function_name!();
        vec![GameEvent::GamePreloaded { game }]
    }};

    ( $final_method:ident ; $($method:ident( $($arg:expr),* $(,)? )),* $(,)? ) => {{
        let builder = GameBuilder::default()
            $( . $method ( $($arg),* ) )* ;

        let mut game = builder. $final_method ();
        *game.name_mut() = function_name!();
        vec![GameEvent::GamePreloaded { game }]
    }};
}
