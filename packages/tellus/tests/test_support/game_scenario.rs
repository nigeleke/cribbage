use std::assert_matches;

use cribbage_domain::prelude::{
    Card, CutForDealOutcome, CutStarterOutcome, DealOutcome, Discard, DiscardOutcome,
    Game as DomainGame, GameError as DomainGameError, GoOutcome, Hands, HasHands, HasPlayState,
    HasRoles, PlayOutcome, PlayState, Player, Roles, ScoreCribOutcome, ScoreDealerOutcome,
    ScorePoneOutcome,
};

use cribbage_tellus::prelude::{
    DeckSource, Game, GameCommand, GameEvent, GameId, GameState, PersistedCard, PersistedCards,
    PersistedDiscard, UserId, Users,
};

use crate::test_support::{EntityFixture, ShuffledDeckSource};

pub struct GameScenario {
    id: GameId,
    pub state: GameState,
    events: Vec<GameEvent>,
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

            match state {
                $variant => {
                    match $action {
                        Ok(outcome) => {
                            let (new_state, new_events) = match outcome {
                                $( $outcome => $result ),+
                            };
                            state = new_state;
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
        match self.state {
            GameState::Starting { users, .. }
            | GameState::Dealing { users, .. }
            | GameState::Discarding { users, .. }
            | GameState::Cutting { users, .. }
            | GameState::Playing { users, .. }
            | GameState::ScoringPone { users, .. }
            | GameState::ScoringDealer { users, .. }
            | GameState::ScoringCrib { users, .. }
            | GameState::Finished { users, .. } => users,
            _ => panic!("no users"),
        }
    }

    pub fn events(&self) -> Vec<GameEvent> {
        self.events.to_vec()
    }

    pub fn given_starting(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.do_create_game()
    }

    pub fn given_dealing(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        let starting = self.given_starting();

        let host = starting.users().user(Player::Player0);
        let guest = starting.users().user(Player::Player1);

        let dealt = starting.do_cut_for_deal(host).do_cut_for_deal(guest);

        match dealt.state {
            GameState::Starting { .. } => Self::default().given_dealing(),
            GameState::Dealing { .. } => dealt,
            _ => unreachable!(),
        }
    }

    pub fn given_discarding(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        let dealing = self.given_dealing();
        dealing.do_deal()
    }

    pub fn given_cutting(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_discarding()
            .advance_discard(Player::Player0)
            .advance_discard(Player::Player1)
    }

    pub fn given_playing(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_cutting().do_cut_starter()
    }

    pub fn given_go(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_playing().advance_to_go()
    }

    pub fn given_played(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_playing().advance_to_played()
    }

    pub fn given_scoring_pone(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_played()
    }

    pub fn given_scoring_dealer(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_scoring_pone().do_score_pone()
    }

    pub fn given_scoring_crib(self) -> Self {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_scoring_dealer().do_score_dealer()
    }

    pub fn given_finished(self) -> (Self, GameCommand) {
        assert_matches!(self.state, GameState::PendingCreation);
        self.given_dealing().advance_to_finished()
    }

    pub fn to_fixture_init(self) -> EntityFixture<Game<ShuffledDeckSource>> {
        let game = Game::new(self.id, ShuffledDeckSource::default());
        EntityFixture::new(game)
    }

    pub fn to_fixture(self) -> EntityFixture<Game<ShuffledDeckSource>> {
        let events = self.events.clone();
        self.to_fixture_init().given(events)
    }

    fn advance_discard(self, player: Player) -> Self {
        assert_matches!(self.state, GameState::Discarding { .. });

        let user = self.users().user(player);
        let hand = self.hand(player);
        let discard = [hand[0], hand[1]];
        self.do_discard(user, discard)
    }

    fn advance_to_go(self) -> Self {
        assert_matches!(self.state, GameState::Playing { .. });

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
            match played.state {
                GameState::Playing { .. } => {
                    if !played.is_finished() {
                        played.advance_to_go()
                    } else {
                        Self::default().given_go()
                    }
                }
                GameState::ScoringPone { .. } | GameState::Finished { .. } => {
                    Self::default().given_go()
                }
                _ => unreachable!(),
            }
        }
    }

    fn advance_to_played(self) -> Self {
        assert_matches!(self.state, GameState::Playing { .. });

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

            match played.state {
                GameState::Playing { .. } => played.advance_to_played(),
                GameState::ScoringPone { .. } => played,
                GameState::Finished { .. } => Self::default().given_played(),
                _ => unreachable!(),
            }
        }
    }

    fn advance_to_finished(self) -> (Self, GameCommand) {
        match &self.state {
            GameState::Dealing { .. } => self.do_deal().advance_to_finished(),

            GameState::Discarding { .. } => self
                .advance_discard(Player::Player0)
                .advance_discard(Player::Player1)
                .advance_to_finished(),

            GameState::Cutting { .. } => {
                let updated = self.do_cut_starter();
                match updated.state {
                    GameState::Finished { .. } => (updated, GameCommand::CutStarter),
                    GameState::Playing { .. } => updated.advance_to_finished(),
                    _ => unreachable!(),
                }
            }

            GameState::Playing { game, users } => {
                let player = game.next_to_play();
                let user = users.user(player);

                let plays = game.legal_plays(player);
                if plays.is_empty() {
                    let updated = self.do_go(user);
                    match updated.state {
                        GameState::Playing { .. } => updated.advance_to_finished(),
                        GameState::ScoringPone { .. } => updated.advance_to_finished(),
                        GameState::Finished { .. } => (updated, GameCommand::Go { user }),
                        _ => unreachable!(),
                    }
                } else {
                    let card = PersistedCard::from(&plays[0]);
                    let updated = self.do_play(user, card);
                    match updated.state {
                        GameState::Playing { .. } | GameState::ScoringPone { .. } => {
                            updated.advance_to_finished()
                        }
                        GameState::Finished { .. } => (updated, GameCommand::Go { user }),
                        _ => unreachable!(),
                    }
                }
            }

            GameState::ScoringPone { .. } => {
                let updated = self.do_score_pone();
                match updated.state {
                    GameState::ScoringDealer { .. } => updated.advance_to_finished(),
                    GameState::Finished { .. } => (updated, GameCommand::ScorePone),
                    _ => unreachable!(),
                }
            }

            GameState::ScoringDealer { .. } => {
                let updated = self.do_score_dealer();
                match updated.state {
                    GameState::ScoringCrib { .. } => updated.advance_to_finished(),
                    GameState::Finished { .. } => (updated, GameCommand::ScoreDealer),
                    _ => unreachable!(),
                }
            }

            GameState::ScoringCrib { .. } => {
                let updated = self.do_score_crib();
                match updated.state {
                    GameState::Dealing { .. } => updated.advance_to_finished(),
                    GameState::Finished { .. } => (updated, GameCommand::ScoreCrib),
                    _ => unreachable!(),
                }
            }

            _ => unreachable!(),
        }
    }

    fn do_create_game(self) -> Self {
        transition!(
            self,
            GameState::PendingCreation,
            {
                let users = Users::new(UserId::new(), UserId::new());
                let deck = ShuffledDeckSource::default().new_deck();
                Ok::<_, DomainGameError>((DomainGame::new(deck.clone()), users, deck))
            },
            (game, users, deck) => (
                GameState::Starting { game, users },
                [GameEvent::GameCreated { users, deck: PersistedCards::from(deck.as_ref()) }]
            )
        )
    }

    fn do_cut_for_deal(self, user: UserId) -> Self {
        transition!(
            self,
            GameState::Starting { game, users },
            {
                let player = users.player(user).expect("valid user required");
                game.cut_for_deal(player)
            },
            CutForDealOutcome::Starting(_, game) => (GameState::Starting { game, users }, [GameEvent::CutForDealMade { user }]),
            CutForDealOutcome::Dealing(_, game) => (GameState::Dealing { game, users }, [GameEvent::CutForDealMade { user }]),
        )
    }

    fn do_deal(self) -> Self {
        transition!(
            self,
            GameState::Dealing { game, users },
            {
                let deck = ShuffledDeckSource::default().new_deck();
                let persisted_deck = PersistedCards::from(deck.as_ref());
                game.deal(deck).map(|outcome| (outcome, persisted_deck))
            },
            (DealOutcome::Discarding(game), persisted_deck) => (
                    GameState::Discarding { game, users },
                    [GameEvent::HandsDealt { deck: persisted_deck }]
            )
        )
    }

    fn do_discard(self, user: UserId, discard: Discard) -> Self {
        transition!(
            self,
            GameState::Discarding { game, users },
            {
                let player = users.player(user).expect("valid user required");
                game.discard(player, discard)
            },
            DiscardOutcome::Discarding(game) => (
                GameState::Discarding { game, users },
                [GameEvent::DiscardMade { user, discard: PersistedDiscard::from(&discard) }]
            ),
            DiscardOutcome::Cutting(game) => (
                GameState::Cutting { game, users },
                [GameEvent::DiscardMade { user, discard: PersistedDiscard::from(&discard) }]
            ),
        )
    }

    fn do_cut_starter(self) -> Self {
        transition!(
            self,
            GameState::Cutting { game, users },
            { game.cut_starter() },
            CutStarterOutcome::Playing(game) => (
                GameState::Playing { game, users },
                [GameEvent::StarterCut]
            ),
            CutStarterOutcome::Finished(game) => (
                GameState::Finished { game, users },
                [GameEvent::StarterCut]
            )
        )
    }

    fn do_play(self, user: UserId, card: PersistedCard) -> Self {
        transition!(
            self,
            GameState::Playing { game, users },
            {
                let player = users.player(user).expect("valid user required");
                let card = Card::try_from(&card).expect("valid card required");
                game.play(player, card)
            },
            PlayOutcome::Playing(game) => (
                GameState::Playing { game, users },
                [GameEvent::CardPlayed { user, card }]
            ),
            PlayOutcome::Scoring(game) => (
                GameState::ScoringPone { game, users },
                [GameEvent::CardPlayed { user, card }]
            ),
            PlayOutcome::Finished(game) => (
                GameState::Finished { game, users },
                [GameEvent::CardPlayed { user, card }]
            )
        )
    }

    fn do_go(self, user: UserId) -> Self {
        transition!(self,
            GameState::Playing { game, users },
            {
                let player = users.player(user).expect("valid user required");
                game.go(player)
            },
            GoOutcome::Playing(game) => (
                GameState::Playing { game, users },
                [GameEvent::GoMade { user }]
            ),
            GoOutcome::Scoring(game) => (
                GameState::ScoringPone { game, users },
                [GameEvent::GoMade { user }]
            ),
            GoOutcome::Finished(game) => (
                GameState::Finished { game, users },
                [GameEvent::GoMade { user }]
            )
        )
    }

    fn do_score_pone(self) -> Self {
        transition!(
            self,
            GameState::ScoringPone { game, users },
            game.score_pone(),
            ScorePoneOutcome::Scoring(game) => (
                GameState::ScoringDealer { game, users },
                [GameEvent::PoneScoreMade]
            ),
            ScorePoneOutcome::Finished(game) => (
                GameState::Finished { game, users },
                [GameEvent::PoneScoreMade]
            )
        )
    }

    fn do_score_dealer(self) -> Self {
        transition!(
            self,
            GameState::ScoringDealer { game, users },
            game.score_dealer(),
            ScoreDealerOutcome::Scoring(game) => (
                GameState::ScoringCrib { game, users },
                [GameEvent::DealerScoreMade]
            ),
            ScoreDealerOutcome::Finished(game) => (
                GameState::Finished { game, users },
                [GameEvent::DealerScoreMade]
            )
        )
    }

    fn do_score_crib(self) -> Self {
        transition!(
            self,
            GameState::ScoringCrib { game, users },
            game.score_crib(),
            ScoreCribOutcome::Dealing(game) => (
                GameState::Dealing { game, users },
                [GameEvent::CribScoreMade]
            ),
            ScoreCribOutcome::Finished(game) => (
                GameState::Finished { game, users },
                [GameEvent::CribScoreMade]
            )
        )
    }
}

impl Default for GameScenario {
    fn default() -> Self {
        Self {
            id: GameId::new(),
            events: Vec::new(),
            state: GameState::PendingCreation,
        }
    }
}

impl HasRoles for GameScenario {
    fn roles(&self) -> &Roles {
        match &self.state {
            GameState::Dealing { game, .. } => game.roles(),
            GameState::Discarding { game, .. } => game.roles(),
            GameState::Cutting { game, .. } => game.roles(),
            GameState::Playing { game, .. } => game.roles(),
            GameState::ScoringPone { game, .. } => game.roles(),
            GameState::ScoringDealer { game, .. } => game.roles(),
            GameState::ScoringCrib { game, .. } => game.roles(),
            GameState::Finished { game, .. } => game.roles(),
            GameState::PendingCreation | GameState::Starting { .. } => {
                panic!("expecting state with roles; got {:?}", self.state)
            }
        }
    }
}

impl HasHands for GameScenario {
    fn hands(&self) -> &Hands {
        match &self.state {
            GameState::Discarding { game, .. } => game.hands(),
            GameState::Cutting { game, .. } => game.hands(),
            GameState::Playing { game, .. } => game.hands(),
            GameState::ScoringPone { game, .. } => game.hands(),
            GameState::ScoringDealer { game, .. } => game.hands(),
            GameState::ScoringCrib { game, .. } => game.hands(),
            GameState::Finished { game, .. } => game.hands(),
            GameState::PendingCreation | GameState::Starting { .. } | GameState::Dealing { .. } => {
                panic!("expecting state with hands; got {:?}", self.state)
            }
        }
    }
}

impl HasPlayState for GameScenario {
    fn play_state(&self) -> &PlayState {
        match &self.state {
            GameState::Playing { game, .. } => game.play_state(),
            GameState::PendingCreation
            | GameState::Starting { .. }
            | GameState::Dealing { .. }
            | GameState::Discarding { .. }
            | GameState::Cutting { .. }
            | GameState::ScoringPone { .. }
            | GameState::ScoringDealer { .. }
            | GameState::ScoringCrib { .. }
            | GameState::Finished { .. } => {
                panic!("expecting state with play_state; got {:?}", self.state)
            }
        }
    }
}
