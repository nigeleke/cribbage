use std::assert_matches;

use cribbage_domain::prelude::{
    Card, CutForDealOutcome, CutStarterOutcome, DealOutcome, Discard, DiscardOutcome,
    DomainError as DomainGameError, Game as DomainGame, GoOutcome, Hands, HasHands, HasPlayState,
    HasRoles, PlayOutcome, PlayState, Player, Roles, ScoreCribOutcome, ScoreDealerOutcome,
    ScorePoneOutcome,
};

use cribbage_tellus::prelude::{
    DeckSource, Game, GameCommand as GC, GameEvent as GE, GameId, GameState as GS, PersistedCard,
    PersistedCards, PersistedDiscard, UserId, Users,
};

use crate::test_support::{EntityFixture, ShuffledDeckSource};

pub struct GameScenario {
    id: GameId,
    pub state: Box<GS>,
    events: Vec<GE>,
}

// Used in advance_to_finish only with respect to stack usage reduction
enum Next {
    Continue(GameScenario),
    Finished(GameScenario, GC),
}

macro_rules! transition {
        (
            $self:expr,
            $variant:pat,
            $action:expr,
            $( $outcome:pat => $result:expr ),+ $(,)?
        ) => {{
            let GameScenario {
                id,
                mut state,
                mut events,
            } = $self;

            match *state {
                $variant => {
                    match $action {
                        Ok(outcome) => {
                            let (new_state, new_events) = match outcome {
                                $( $outcome => $result ),+
                            };
                            state = Box::new(new_state);
                            events.extend(new_events);
                        }
                        Err(e) => panic!("domain error: {e}"),
                    };
                }
                other => panic!("invalid state for action: {} while {:?}", stringify!($action), other),
            };

            GameScenario { id, state, events }
        }};
    }

impl GameScenario {
    pub fn users(&self) -> Users {
        match *self.state {
            GS::Starting { users, .. }
            | GS::Dealing { users, .. }
            | GS::Discarding { users, .. }
            | GS::Cutting { users, .. }
            | GS::Playing { users, .. }
            | GS::ScoringPone { users, .. }
            | GS::ScoringDealer { users, .. }
            | GS::ScoringCrib { users, .. }
            | GS::Finished { users, .. } => users,
            _ => panic!("no users"),
        }
    }

    pub fn events(&self) -> Vec<GE> {
        self.events.to_vec()
    }

    pub fn given_starting(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.do_create_game()
    }

    pub fn given_dealing(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        let starting = self.given_starting();

        let host = starting.users().user(Player::Player0);
        let guest = starting.users().user(Player::Player1);

        let dealt = starting.do_cut_for_deal(host).do_cut_for_deal(guest);

        match *dealt.state {
            GS::Starting { .. } => Self::default().given_dealing(),
            GS::Dealing { .. } => dealt,
            _ => unreachable!(),
        }
    }

    pub fn given_discarding(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        let dealing = self.given_dealing();
        dealing.do_deal()
    }

    pub fn given_cutting(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_discarding()
            .advance_discard(Player::Player0)
            .advance_discard(Player::Player1)
    }

    pub fn given_playing(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_cutting().do_cut_starter()
    }

    pub fn given_go(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_playing().advance_to_go()
    }

    pub fn given_played(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_playing().advance_to_played()
    }

    pub fn given_scoring_pone(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_played()
    }

    pub fn given_scoring_dealer(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_scoring_pone().do_score_pone()
    }

    pub fn given_scoring_crib(self) -> Self {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_scoring_dealer().do_score_dealer()
    }

    pub fn given_finished(self) -> (Self, GC) {
        assert_matches!(*self.state, GS::PendingCreation);
        self.given_dealing().advance_to_finished()
    }

    pub fn to_fixture_init(self) -> EntityFixture<Game> {
        let game = Game::new(self.id);
        EntityFixture::new(game)
    }

    pub fn to_fixture(self) -> EntityFixture<Game> {
        let events = self.events.clone();
        self.to_fixture_init().given(events)
    }

    fn advance_discard(self, player: Player) -> Self {
        assert_matches!(*self.state, GS::Discarding { .. });

        let user = self.users().user(player);
        let hand = self.hand(player);
        let discard = [hand[0], hand[1]];
        self.do_discard(user, discard)
    }

    fn advance_to_go(self) -> Self {
        assert_matches!(*self.state, GS::Playing { .. });

        let player = self.next_to_play();
        if self.legal_plays(player).is_empty() {
            // A sequence of plays to a go is not guaranteed. It's possible that
            // all cards are low values, and plays continue until end-of-play, in which
            // case we try again with another shuffled deck.
            if !self.is_finished() {
                self
            } else {
                Self::default().given_go()
            }
        } else {
            let user = self.users().user(player);
            let card = self.legal_plays(player)[0];
            let played = self.do_play(user, PersistedCard::from(&card));

            // If it was the last legal play, then there won't be a go, so
            // try again with another shuffled deck.
            match *played.state {
                GS::Playing { .. } => {
                    if !played.is_finished() {
                        played.advance_to_go()
                    } else {
                        Self::default().given_go()
                    }
                }
                GS::ScoringPone { .. } | GS::Finished { .. } => Self::default().given_go(),
                _ => unreachable!(),
            }
        }
    }

    fn advance_to_played(self) -> Self {
        assert_matches!(*self.state, GS::Playing { .. });

        let player = self.next_to_play();
        if self.is_finished() {
            self
        } else {
            let user = self.users().user(player);

            let played = if self.legal_plays(player).is_empty() {
                self.do_go(user)
            } else {
                let card = self.legal_plays(player)[0];
                self.do_play(user, PersistedCard::from(&card))
            };

            match *played.state {
                GS::Playing { .. } => played.advance_to_played(),
                GS::ScoringPone { .. } => played,
                GS::Finished { .. } => Self::default().given_played(),
                _ => unreachable!(),
            }
        }
    }

    fn advance_to_finished(self) -> (Self, GC) {
        let mut scenario = self;

        loop {
            match scenario.do_advance_to_finished_step() {
                Next::Continue(next) => scenario = next,
                Next::Finished(scenario, command) => {
                    break (scenario, command);
                }
            }
        }
    }

    fn do_advance_to_finished_step(self) -> Next {
        match &*self.state {
            GS::Dealing { .. } => Next::Continue(self.do_deal()),

            GS::Discarding { .. } => Next::Continue(
                self.advance_discard(Player::Player0)
                    .advance_discard(Player::Player1),
            ),

            GS::Cutting { .. } => {
                let updated = self.do_cut_starter();
                match *updated.state {
                    GS::Finished { .. } => Next::Finished(updated, GC::CutStarter),
                    GS::Playing { .. } => Next::Continue(updated),
                    _ => unreachable!(),
                }
            }

            GS::Playing { game, users } => {
                let player = game.next_to_play();
                let user = users.user(player);

                let plays = game.legal_plays(player);
                if plays.is_empty() {
                    let updated = self.do_go(user);
                    match *updated.state {
                        GS::Playing { .. } => Next::Continue(updated),
                        GS::ScoringPone { .. } => Next::Continue(updated),
                        GS::Finished { .. } => Next::Finished(updated, GC::Go { user }),
                        _ => unreachable!(),
                    }
                } else {
                    let card = PersistedCard::from(&plays[0]);
                    let updated = self.do_play(user, card);
                    match *updated.state {
                        GS::Playing { .. } | GS::ScoringPone { .. } => Next::Continue(updated),
                        GS::Finished { .. } => Next::Finished(updated, GC::PlayCard { user, card }),
                        _ => unreachable!(),
                    }
                }
            }

            GS::ScoringPone { .. } => {
                let updated = self.do_score_pone();
                match *updated.state {
                    GS::ScoringDealer { .. } => Next::Continue(updated),
                    GS::Finished { .. } => Next::Finished(updated, GC::ScorePone),
                    _ => unreachable!(),
                }
            }

            GS::ScoringDealer { .. } => {
                let updated = self.do_score_dealer();
                match *updated.state {
                    GS::ScoringCrib { .. } => Next::Continue(updated),
                    GS::Finished { .. } => Next::Finished(updated, GC::ScoreDealer),
                    _ => unreachable!(),
                }
            }

            GS::ScoringCrib { .. } => {
                let updated = self.do_score_crib();
                match *updated.state {
                    GS::Dealing { .. } => Next::Continue(updated),
                    GS::Finished { .. } => Next::Finished(updated, GC::ScoreCrib),
                    _ => unreachable!(),
                }
            }

            _ => unreachable!(),
        }
    }

    fn do_create_game(self) -> Self {
        transition!(
            self,
            GS::PendingCreation,
            {
                let users = Users::new(UserId::new(), UserId::new());
                let deck = ShuffledDeckSource::default().new_deck();
                Ok::<_, DomainGameError>((DomainGame::new(deck.clone()), users, deck))
            },
            (game, users, deck) => (
                GS::Starting { game, users },
                [GE::GameCreated { users, deck: PersistedCards::from(deck.as_ref()) }]
            )
        )
    }

    fn do_cut_for_deal(self, user: UserId) -> Self {
        transition!(
            self,
            GS::Starting { game, users },
            {
                let player = users.player(user).expect("valid user required");
                game.cut_for_deal(player)
            },
            CutForDealOutcome::Starting(_, game) => (GS::Starting { game, users }, [GE::CutForDealMade { user }]),
            CutForDealOutcome::Dealing(_, game) => (GS::Dealing { game, users }, [GE::CutForDealMade { user }]),
        )
    }

    fn do_deal(self) -> Self {
        transition!(
            self,
            GS::Dealing { game, users },
            {
                let deck = ShuffledDeckSource::default().new_deck();
                let persisted_deck = PersistedCards::from(deck.as_ref());
                game.deal(deck).map(|outcome| (outcome, persisted_deck))
            },
            (DealOutcome::Discarding(game), persisted_deck) => (
                    GS::Discarding { game, users },
                    [GE::HandsDealt { deck: persisted_deck }]
            )
        )
    }

    fn do_discard(self, user: UserId, discard: Discard) -> Self {
        transition!(
            self,
            GS::Discarding { game, users },
            {
                let player = users.player(user).expect("valid user required");
                game.discard(player, discard)
            },
            DiscardOutcome::Discarding(game) => (
                GS::Discarding { game, users },
                [GE::DiscardMade { user, discard: PersistedDiscard::from(&discard) }]
            ),
            DiscardOutcome::Cutting(game) => (
                GS::Cutting { game, users },
                [GE::DiscardMade { user, discard: PersistedDiscard::from(&discard) }]
            ),
        )
    }

    fn do_cut_starter(self) -> Self {
        transition!(
            self,
            GS::Cutting { game, users },
            { game.cut_starter() },
            CutStarterOutcome::Playing(game) => (
                GS::Playing { game, users },
                [GE::StarterCut]
            ),
            CutStarterOutcome::Finished(game) => (
                GS::Finished { game, users },
                [GE::StarterCut]
            )
        )
    }

    fn do_play(self, user: UserId, card: PersistedCard) -> Self {
        transition!(
            self,
            GS::Playing { game, users },
            {
                let player = users.player(user).expect("valid user required");
                let card = Card::try_from(&card).expect("valid card required");
                game.play(player, card)
            },
            PlayOutcome::Playing(game) => (
                GS::Playing { game, users },
                [GE::CardPlayed { user, card }]
            ),
            PlayOutcome::Scoring(game) => (
                GS::ScoringPone { game, users },
                [GE::CardPlayed { user, card }]
            ),
            PlayOutcome::Finished(game) => (
                GS::Finished { game, users },
                [GE::CardPlayed { user, card }]
            )
        )
    }

    fn do_go(self, user: UserId) -> Self {
        transition!(self,
            GS::Playing { game, users },
            {
                let player = users.player(user).expect("valid user required");
                game.go(player)
            },
            GoOutcome::Playing(game) => (
                GS::Playing { game, users },
                [GE::GoMade { user }]
            ),
            GoOutcome::Scoring(game) => (
                GS::ScoringPone { game, users },
                [GE::GoMade { user }]
            ),
            GoOutcome::Finished(game) => (
                GS::Finished { game, users },
                [GE::GoMade { user }]
            )
        )
    }

    fn do_score_pone(self) -> Self {
        transition!(
            self,
            GS::ScoringPone { game, users },
            game.score_pone(),
            ScorePoneOutcome::Scoring(game) => (
                GS::ScoringDealer { game, users },
                [GE::PoneScoreMade]
            ),
            ScorePoneOutcome::Finished(game) => (
                GS::Finished { game, users },
                [GE::PoneScoreMade]
            )
        )
    }

    fn do_score_dealer(self) -> Self {
        transition!(
            self,
            GS::ScoringDealer { game, users },
            game.score_dealer(),
            ScoreDealerOutcome::Scoring(game) => (
                GS::ScoringCrib { game, users },
                [GE::DealerScoreMade]
            ),
            ScoreDealerOutcome::Finished(game) => (
                GS::Finished { game, users },
                [GE::DealerScoreMade]
            )
        )
    }

    fn do_score_crib(self) -> Self {
        transition!(
            self,
            GS::ScoringCrib { game, users },
            game.score_crib(),
            ScoreCribOutcome::Dealing(game) => (
                GS::Dealing { game, users },
                [GE::CribScoreMade]
            ),
            ScoreCribOutcome::Finished(game) => (
                GS::Finished { game, users },
                [GE::CribScoreMade]
            )
        )
    }
}

impl Default for GameScenario {
    fn default() -> Self {
        Self {
            id: GameId::new(),
            events: Vec::new(),
            state: Box::new(GS::PendingCreation),
        }
    }
}

impl HasRoles for GameScenario {
    fn roles(&self) -> &Roles {
        match &*self.state {
            GS::Dealing { game, .. } => game.roles(),
            GS::Discarding { game, .. } => game.roles(),
            GS::Cutting { game, .. } => game.roles(),
            GS::Playing { game, .. } => game.roles(),
            GS::ScoringPone { game, .. } => game.roles(),
            GS::ScoringDealer { game, .. } => game.roles(),
            GS::ScoringCrib { game, .. } => game.roles(),
            GS::Finished { game, .. } => game.roles(),
            GS::PendingCreation | GS::Starting { .. } => {
                panic!("expecting state with roles; got {:?}", *self.state)
            }
        }
    }
}

impl HasHands for GameScenario {
    fn hands(&self) -> &Hands {
        match &*self.state {
            GS::Discarding { game, .. } => game.hands(),
            GS::Cutting { game, .. } => game.hands(),
            GS::Playing { game, .. } => game.hands(),
            GS::ScoringPone { game, .. } => game.hands(),
            GS::ScoringDealer { game, .. } => game.hands(),
            GS::ScoringCrib { game, .. } => game.hands(),
            GS::Finished { game, .. } => game.hands(),
            GS::PendingCreation | GS::Starting { .. } | GS::Dealing { .. } => {
                panic!("expecting state with hands; got {:?}", *self.state)
            }
        }
    }
}

impl HasPlayState for GameScenario {
    fn play_state(&self) -> &PlayState {
        match &*self.state {
            GS::Playing { game, .. } => game.play_state(),
            GS::PendingCreation
            | GS::Starting { .. }
            | GS::Dealing { .. }
            | GS::Discarding { .. }
            | GS::Cutting { .. }
            | GS::ScoringPone { .. }
            | GS::ScoringDealer { .. }
            | GS::ScoringCrib { .. }
            | GS::Finished { .. } => {
                panic!("expecting state with play_state; got {:?}", *self.state)
            }
        }
    }
}
