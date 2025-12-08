use std::str::FromStr;

use crate::{card, cards, crib, domain::*, hand};

#[derive(Debug)]
pub struct GameBuilder {
    dealer: usize,
    cuts: Vec<Card>,
    positions: Vec<usize>,
    hands: Vec<Hand>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
    go_status: GoStatus,
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
        self.positions.push(points0);
        self.positions.push(points1);
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

    pub fn with_go(mut self) -> Self {
        self.go_status = GoStatus::Called;
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

        game.apply_event(GameEvent::ComputerGameCreated {
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
        let positions = [
            Position::new(*self.positions.get(0).unwrap_or(&0)),
            Position::new(*self.positions.get(1).unwrap_or(&0)),
        ];
        Scoreboard::new(positions)
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

        *play_state.go_status_mut() = self.go_status;
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

    pub fn as_starting(self) -> Game {
        let starting = Starting::new(
            self.domain_cuts(),
            self.domain_deck(),
            self.domain_pending(),
        );
        Self::new_game(starting.wrap())
    }

    pub fn as_discarding(mut self) -> Game {
        self.cut.into_iter().for_each(|c| self.deck.add(c));

        let discarding = Discarding::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_deck(),
            self.domain_pending(),
        );

        Self::new_game(discarding.wrap())
    }

    pub fn as_playing(self, next_to_play: usize) -> Game {
        let playing = Playing::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_play_state(next_to_play),
            self.domain_crib(),
            self.domain_cut(),
            self.domain_pending(),
        );
        Self::new_game(playing.wrap())
    }

    pub fn as_scoring_pone(self) -> Game {
        let pone = 1 - self.dealer;
        let pegging = Pegging::new(
            Player::from(pone),
            ScoreSheet::hand(&self.hands[pone], self.domain_cut()),
        );
        let scoring = ScoringPone::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
            pegging,
            self.domain_pending(),
        );
        Self::new_game(scoring.wrap())
    }

    pub fn as_scoring_dealer(self) -> Game {
        let dealer = self.dealer;
        let pegging = Pegging::new(
            Player::from(dealer),
            ScoreSheet::hand(&self.hands[dealer], self.domain_cut()),
        );
        let scoring = ScoringDealer::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
            pegging,
            self.domain_pending(),
        );
        Self::new_game(scoring.wrap())
    }

    pub fn as_scoring_crib(self) -> Game {
        let dealer = self.dealer;
        let pegging = Pegging::new(
            Player::from(dealer),
            ScoreSheet::crib(&self.crib, self.domain_cut()),
        );
        let scoring = ScoringCrib::new(
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
            pegging,
            self.domain_pending(),
        );
        Self::new_game(scoring.wrap())
    }

    pub fn as_finished(self) -> Game {
        let finished = Finished::new(
            self.domain_winner(),
            self.domain_scoreboard(),
            self.domain_roles(),
            self.domain_hands(),
            self.domain_crib(),
            self.domain_cut(),
        );
        Self::new_game(finished.wrap())
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            deck: Deck::shuffled_pack(),
            dealer: 0,
            cuts: Vec::default(),
            positions: Vec::new(),
            hands: Vec::default(),
            current_plays: Vec::default(),
            previous_plays: Vec::default(),
            go_status: GoStatus::default(),
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

#[macro_export]
macro_rules! find_then {
    ($slice:expr, $pat:pat => $assert:block) => {
        #[allow(unused)]
        if let Some(event) = $slice.iter().find(|e| matches!(e, $pat)) {
            if let $pat = event {
                $assert
            } else {
                unreachable!()
            }
        } else {
            panic!("expected {} not found", stringify!($pat));
        }
    };
}

#[macro_export]
macro_rules! assert_state_then {
    ($state:expr, $pat:pat $(if $guard:expr)? => $assert:block) => {{
        match $state {
            $pat $(if $guard)? => $assert,
            other => panic!("unexpected state: {:?}", other),
        }
    }};
}

pub fn __private_game_test_impl(
    function_name: String,
    given: &[GameEvent],
    when: GameCommand,
    then_events: Option<impl Fn(&[GameEvent])>,
    then_state: Option<impl Fn(&State)>,
    then_error: Option<DomainError>,
) {
    let is_happy_path_tests = then_events.is_some() || then_state.is_some();
    let is_error_path_tests = then_error.is_some();

    assert!(
        (is_happy_path_tests && !is_error_path_tests)
            || (is_error_path_tests && !is_happy_path_tests)
    );

    let mut game = Game::from(given);

    let result = cqrs_es::test::TestFramework::<Game>::with(GameServices)
        .given(Vec::from(given))
        .when(when)
        .inspect_result();

    match result {
        Ok(events) if is_happy_path_tests => {
            then_events.iter().for_each(|f| f(events.as_slice()));
            game.apply_events(&events);
            let state = game.state();
            then_state.iter().for_each(|f| f(state));
        }
        Ok(events) => panic!("unexpected result in {function_name}: {events:?}"),
        Err(error) if is_error_path_tests => {
            then_error
                .into_iter()
                .for_each(|e| assert_eq!(error, e, "in function {function_name}"));
        }
        Err(error) => panic!("unexpected result in {function_name}: {error:?}"),
    };
}

#[macro_export]
macro_rules! game_test {
    {
        given: $given:expr,
        when: $when:expr,
        then_events: $events_fn:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            $given,
            $when,
            Some($events_fn),
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        given: $given:expr,
        when: $when:expr,
        then_events: $events_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            $given,
            $when,
            Some($events_fn),
            None::<fn(&State)>,
            None::<DomainError>
        );
    }};

    {
        given: $given:expr,
        when: $when:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            $given,
            $when,
            None::<fn(&[GameEvent])>,
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        given: $given:expr,
        when: $when:expr,
        then_error: $error:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            $given,
            $when,
            None::<fn(&[GameEvent])>,
            None::<fn(&State)>,
            Some($error)
        );
    }};

    {
        when: $when:expr,
        then_events: $events_fn:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            &[],
            $when,
            Some($events_fn),
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        when: $when:expr,
        then_events: $events_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            &[],
            $when,
            Some($events_fn),
            None::<fn(&State)>,
            None::<DomainError>
        );
    }};

    {
        when: $when:expr,
        then_state: $state_fn:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            &[],
            $when,
            None::<fn(&[GameEvent])>,
            Some($state_fn),
            None::<DomainError>
        );
    }};

    {
        when: $when:expr,
        then_error: $error:expr
    } => {{
        $crate::domain::test::__private_game_test_impl(
            function_name!(),
            &[],
            $when,
            None::<fn(&[GameEvent])>,
            None::<fn(&State)>,
            Some($error)
        );
    }};

}
