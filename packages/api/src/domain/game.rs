use crate::{
    Crib, Deck, Discarding, Event, Finished, GameId, PLAYER0, PLAYER1, Pending, PlayState, Playing,
    Roles, ScoringPone, Starting, State, UserId, constants::PLAYER_COUNT,
};
use eventsourced::EventSourced;
use serde::{Deserialize, Serialize};

/// Represents a game session, including host and guest players, game metadata, and state.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    /// Unique identifier for the game.
    id: GameId,

    /// The user who created or is hosting the game.
    host: UserId,

    /// Optional user ID of the guest player, if one has joined.
    guest: Option<UserId>,

    /// The name or title of the game.
    name: String,

    /// The current state of the game.
    state: State,
}

impl Game {
    /// Returns a reference to the game’s unique ID.
    pub fn id(&self) -> &GameId {
        &self.id
    }

    /// Returns a reference to the host user’s ID.
    pub fn host(&self) -> &UserId {
        &self.host
    }

    /// Returns an optional reference to the guest user’s ID, if a guest has joined.
    pub fn guest(&self) -> Option<&UserId> {
        self.guest.as_ref()
    }

    /// Return a reference to the game's current state.
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl EventSourced for Game {
    type Id = GameId;
    type Event = Event;

    const TYPE_NAME: &'static str = stringify!(Game);

    fn handle_event(mut self, event: Self::Event) -> Self {
        println!("Game::handle_event: {event:?}");
        match event {
            Self::Event::LobbyGameCreated {
                game_id,
                host,
                name,
            } => self.initial(game_id, host, None, name),
            Self::Event::LobbyGameJoined { game_id: _, guest } => {
                self.guest = Some(guest);
                self
            }
            Self::Event::ComputerGameStarted {
                game_id,
                users,
                name,
            } => self.initial(game_id, users[0], Some(users[1]), name),
            Self::Event::RoundStarted {
                game_id: _,
                dealer,
                scoreboard,
            } => {
                let pone = dealer.opponent();
                let roles = Roles::new(dealer, pone);
                let mut deck = Deck::shuffled_pack();
                let hands = deck.deal(PLAYER_COUNT);
                let hands = [hands[0].clone(), hands[1].clone()];
                let crib = Crib::default();
                let discarding = Discarding::new(scoreboard, roles, hands, crib, deck);
                self.state = State::Discarding(discarding);
                self
            }
            Self::Event::CardCutAtStartOfPlay { game_id: _, cut } => {
                if let State::Discarding(discarding) = self.state {
                    let (scoreboard, roles, hands, crib, mut deck, _pending) =
                        discarding.into_parts();
                    deck.remove(cut);
                    let next_to_play = roles.pone().player();
                    let play_state = PlayState::new(next_to_play)
                        .with_pending_plays(PLAYER0, hands[PLAYER0].as_ref())
                        .with_pending_plays(PLAYER1, hands[PLAYER1].as_ref());
                    let playing = Playing::new(scoreboard, roles, hands, play_state, crib, cut);
                    self.state = State::Playing(playing);
                }
                self.apply_event(event)
            }
            Self::Event::CardPlayed {
                game_id: _,
                player: _,
                card: _,
            } => {
                self = self.apply_event(event);
                println!("Game::CardPlayed {}", self.state);
                if let State::Playing(ref playing) = self.state {
                    if playing.play_state().all_cards_are_played() {
                        let (scoreboard, roles, _, mut play_state, crib, cut) =
                            playing.clone().into_parts();
                        let hands = play_state.finish_plays();
                        let pending = Pending::default();

                        if let Some(winner) = scoreboard.winner() {
                            let finished =
                                Finished::new(winner, scoreboard, roles, hands, crib, cut);
                            self.state = State::Finished(finished);
                        } else {
                            let scoring =
                                ScoringPone::new(scoreboard, roles, hands, crib, cut, pending);
                            self.state = State::ScoringPone(scoring);
                        }
                    }
                }
                self
            }
            Self::Event::WinnerDeclared { game_id: _, winner } => {
                if let Some((scoreboard, roles, hands, crib, cut)) =
                    if let State::Playing(ref playing) = self.state {
                        let (scoreboard, roles, hands, _, crib, cut) = playing.clone().into_parts();
                        Some((scoreboard, roles, hands, crib, cut))
                    } else {
                        None
                    }
                {
                    let finished = Finished::new(winner, scoreboard, roles, hands, crib, cut);
                    self.state = State::Finished(finished);
                }
                self
            }
            _ => self.apply_event(event),
        }
    }
}

impl Game {
    fn initial(mut self, id: GameId, host: UserId, guest: Option<UserId>, name: String) -> Self {
        self.id = id;
        self.host = host;
        self.guest = guest;
        self.name = name;
        let starting = Starting::default();
        self.state = State::Starting(starting);
        self
    }

    fn apply_event(mut self, event: Event) -> Self {
        println!("**** Applying {event:?} to substate {}", self.state);
        match &mut self.state {
            State::Starting(starting) => *starting = starting.clone().handle_event(event),
            State::Discarding(discarding) => *discarding = discarding.clone().handle_event(event),
            State::Playing(playing) => *playing = playing.clone().handle_event(event),
            State::ScoringPone(scoring) => *scoring = scoring.clone().handle_event(event),
            State::Finished(finished) => *finished = finished.clone().handle_event(event),
        };

        self
    }
}

#[cfg(test)]
impl From<State> for Game {
    fn from(state: State) -> Self {
        let id = GameId::new();
        let host = UserId::new();
        let guest = Some(UserId::new());
        let name = format!("test_game_{}_{}", state.as_ref(), chrono::Utc::now());
        Self {
            id,
            host,
            guest,
            name,
            state,
        }
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use super::*;
    use crate::prettify;

    /// # [Cribbage Rules](https://www.officialgamerules.org/cribbage)
    #[allow(clippy::expect_used)]

    /// ## Number of Players
    ///
    /// Two or three people can play. Or four people can play two against two as partners. But
    /// Cribbage is basically best played by two people, and the rules that follow are for that
    /// number.
    #[coverage(off)]
    mod players {
        use super::*;
        use crate::{Error, HostGame, JoinGame, PlayComputer, test::GameTestFramework};

        #[test]
        fn a_user_can_host_game() {
            let game_id = GameId::new();
            let host = UserId::new();
            let command = HostGame::new(host);

            let test = GameTestFramework::new(game_id, Game::default()).when(command);
            let event = test.event();

            if let Event::LobbyGameCreated {
                game_id: _,
                host: actual_host,
                name: _,
            } = event
            {
                assert_eq!(actual_host, &host);
            } else {
                panic!("unexpected event")
            };
        }

        #[test]
        fn a_user_can_play_the_computer() {
            let game_id = GameId::new();
            let host = UserId::new();
            let computer = UserId::default();
            let command = PlayComputer::new(host);

            let test = GameTestFramework::new(game_id, Game::default()).when(command);
            let event = test.event();

            if let Event::ComputerGameStarted {
                game_id: _,
                users: [actual_host, actual_computer],
                name: _,
            } = event
            {
                assert_eq!(actual_host, &host);
                assert_eq!(actual_computer, &computer);
            } else {
                panic!("unexpected event")
            };
        }

        #[test]
        fn a_user_can_join_lobby_game() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = prettify!(a_user_can_join_lobby_game);

            let preconditions = vec![Event::LobbyGameCreated {
                game_id,
                host,
                name,
            }];
            let command = JoinGame::new(game_id, guest);

            GameTestFramework::new(game_id, Game::default())
                .given(preconditions)
                .when(command)
                .expect_event(Event::LobbyGameJoined { game_id, guest });
        }

        #[test]
        fn a_user_cannot_join_active_game() {
            let game_id = GameId::new();
            let host = UserId::new();
            let name = prettify!(a_user_cannot_join_active_game);
            let guest = UserId::new();

            let preconditions = vec![
                Event::LobbyGameCreated {
                    game_id,
                    host,
                    name: name.clone(),
                },
                Event::ComputerGameStarted {
                    game_id,
                    users: [host, guest],
                    name,
                },
            ];
            let command = JoinGame::new(game_id, guest);

            GameTestFramework::new(game_id, Game::default())
                .given(preconditions)
                .when(command)
                .expect_error(Error::NotPermitted(prettify!(JoinGame)));
        }

        #[test]
        fn a_different_user_must_join_lobby_game() {
            let game_id = GameId::new();
            let host = UserId::new();
            let name = prettify!(a_different_user_must_join_game);
            let guest = host;

            let preconditions = vec![Event::LobbyGameCreated {
                game_id,
                host,
                name,
            }];
            let command = JoinGame::new(game_id, guest);

            GameTestFramework::new(game_id, Game::default())
                .given(preconditions)
                .when(command)
                .expect_error(Error::InvalidOpponent(guest));
        }
    }

    /// ## The Pack
    ///
    /// The standard 52-card pack is used.
    ///
    /// Rank of Cards: K (high), Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A.
    mod deck {
        use super::*;
        use crate::STANDARD_DECK_SIZE;

        #[test]
        fn use_a_standard_pack_of_cards() {
            let starting = Starting::default();
            let deck = starting.deck();
            assert_eq!(deck.len(), STANDARD_DECK_SIZE);
        }
    }

    /// ## The Draw, Shuffle and Cut
    ///
    /// From a shuffled pack face down, each player cuts a card, leaving at least four cards at
    /// either end of the pack.
    ///
    /// If both players cut cards of the same rank, each draws again. The player with the lower card
    /// deals the first hand. Thereafter, the turn to deal alternates between the two players,
    /// except that the loser of the game deals first if another game is played. The dealer has the
    /// right to shuffle last, and he presents the cards to the non-dealer for the cut prior to the
    /// deal. (In some games, there is no cut at this time.)
    mod deal_cut {
        use super::*;
        use crate::{
            CutForDeal, Dealer, Deck, HasFace, HasValue, PLAYER0, PLAYER1, RequestRedraw,
            Scoreboard, StartRound, test::GameTestFramework,
        };
        use std::cmp::Ordering;

        #[test]
        fn user_must_confirm_the_cut_1() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = prettify!(user_must_confirm_the_cut_1);

            let preconditions = vec![
                Event::LobbyGameCreated {
                    game_id,
                    host,
                    name,
                },
                Event::LobbyGameJoined { game_id, guest },
            ];
            let command = CutForDeal::new(game_id, PLAYER0);

            let test = GameTestFramework::new(game_id, Game::default())
                .given(preconditions)
                .when(command);
            let event = test.event();

            if let Event::CardCutForDeal {
                game_id: actual_game_id,
                player: actual_player,
                cut: _,
            } = event
            {
                assert_eq!(actual_game_id, &game_id);
                assert_eq!(actual_player, &PLAYER0);
            } else {
                panic!("unexpected event")
            };

            let reply = test.reply();
            assert_eq!(reply, &false);
        }

        #[test]
        fn user_must_confirm_the_cut_2() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = prettify!(user_must_confirm_the_cut_2);

            let mut deck = Deck::shuffled_pack();
            let cut = deck.cut();

            let preconditions = vec![
                Event::LobbyGameCreated {
                    game_id,
                    host,
                    name,
                },
                Event::LobbyGameJoined { game_id, guest },
                Event::CardCutForDeal {
                    game_id,
                    player: PLAYER0,
                    cut,
                },
            ];
            let command = CutForDeal::new(game_id, PLAYER1);

            let test = GameTestFramework::new(game_id, Game::default())
                .given(preconditions)
                .when(command);
            let event = test.event();

            if let Event::CardCutForDeal {
                game_id: actual_game_id,
                player: actual_player,
                cut: _,
            } = event
            {
                assert_eq!(actual_game_id, &game_id);
                assert_eq!(actual_player, &PLAYER1);
            } else {
                panic!("unexpected event");
            };

            let reply = test.reply();
            assert_eq!(reply, &true);
        }

        #[test]
        fn start_game_with_lowest_cut_as_dealer() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = prettify!(start_game_with_lowest_cut_as_dealer);

            let mut preconditions = vec![Event::ComputerGameStarted {
                game_id,
                users: [host, guest],
                name,
            }];

            let dealer = loop {
                let mut deck = Deck::shuffled_pack();
                let cut1 = deck.cut();
                let cut2 = deck.cut();

                preconditions.push(Event::CardCutForDeal {
                    game_id,
                    player: PLAYER0,
                    cut: cut1,
                });

                preconditions.push(Event::CardCutForDeal {
                    game_id,
                    player: PLAYER1,
                    cut: cut2,
                });

                match cut1.face().value().cmp(&cut2.face().value()) {
                    Ordering::Less => break Dealer::from(PLAYER0),
                    Ordering::Greater => break Dealer::from(PLAYER1),
                    Ordering::Equal => {
                        preconditions.pop();
                        preconditions.pop();
                    }
                };
            };

            let scoreboard = Scoreboard::default();
            let command = StartRound::new(game_id, dealer, scoreboard.clone());

            let test = GameTestFramework::new(game_id, Game::default())
                .given(preconditions)
                .when(command);
            let event = test.event();

            let Event::RoundStarted {
                game_id: actual_game_id,
                dealer: actual_dealer,
                scoreboard: actual_scoreboard,
            } = event
            else {
                panic!("unexpected event")
            };

            assert_eq!(actual_game_id, &game_id);
            assert_eq!(actual_dealer, &dealer);
            assert_eq!(actual_scoreboard, &scoreboard);
        }

        #[test]
        fn redraw_when_cut_for_deal_tied() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = prettify!(redraw_when_cut_for_deal_tied);

            let mut preconditions = vec![Event::ComputerGameStarted {
                game_id,
                users: [host, guest],
                name,
            }];

            loop {
                let mut deck = Deck::shuffled_pack();
                let cut1 = deck.cut();
                let cut2 = deck.cut();

                preconditions.push(Event::CardCutForDeal {
                    game_id,
                    player: PLAYER0,
                    cut: cut1,
                });

                preconditions.push(Event::CardCutForDeal {
                    game_id,
                    player: PLAYER1,
                    cut: cut2,
                });

                match cut1.face().value().cmp(&cut2.face().value()) {
                    Ordering::Less | Ordering::Greater => {
                        preconditions.pop();
                        preconditions.pop();
                    }
                    Ordering::Equal => break,
                };
            }

            let command = RequestRedraw::new(game_id);

            let test = GameTestFramework::new(game_id, Game::default())
                .given(preconditions)
                .when(command);
            let event = test.event();

            let Event::RedrawRequested {
                game_id: actual_game_id,
            } = event
            else {
                panic!("unexpected event")
            };

            assert_eq!(actual_game_id, &game_id);
        }
    }

    /// ## The Deal
    ///
    /// The dealer distributes six cards face down to his opponent and himself, beginning with the
    /// opponent.
    mod deal {
        use super::*;
        use crate::{
            Dealer, STANDARD_DECK_SIZE, Scoreboard, constants::CARDS_DEALT_PER_HAND,
            test::GameTestFramework,
        };

        #[test]
        fn dealer_deals_six_cards_each() {
            let game_id = GameId::new();
            let host = UserId::new();
            let guest = UserId::new();
            let name = prettify!(dealer_deals_six_cards_each);

            let dealer = Dealer::from(PLAYER0);
            let scoreboard = Scoreboard::default();

            let preconditions = vec![
                Event::ComputerGameStarted {
                    game_id,
                    users: [host, guest],
                    name,
                },
                Event::RoundStarted {
                    game_id,
                    dealer,
                    scoreboard,
                },
            ];

            let test = GameTestFramework::new(game_id, Game::default()).given(preconditions);
            let entity = test.entity();

            let State::Discarding(discarding) = &entity.state else {
                panic!("unexpected state")
            };

            let deck = discarding.deck();
            let player0_hand = discarding.hand(PLAYER0);
            let player1_hand = discarding.hand(PLAYER1);

            assert_eq!(
                deck.len(),
                STANDARD_DECK_SIZE - (CARDS_DEALT_PER_HAND * PLAYER_COUNT)
            );
            assert_eq!(player0_hand.len(), CARDS_DEALT_PER_HAND);
            assert_eq!(player1_hand.len(), CARDS_DEALT_PER_HAND);
            assert!(deck.contains_none(player0_hand.as_ref()));
            assert!(deck.contains_none(player1_hand.as_ref()));
        }
    }

    /// ## Object of the Game

    /// The goal is to be the first player to score 121 points. (Some games are to 61 points.)
    /// Players earn points during play and for making various card combinations.
    mod object_of_the_game {}

    /// ## The Crib
    ///
    /// Each player looks at his six cards and "lays away" (discards) two of them face down to
    /// reduce the hand to four. The four cards laid away together constitute "the crib". The crib
    /// belongs to the dealer, but these cards are not exposed or used until after the hands have
    /// been played.
    mod the_crib {
        use crate::{card, display::format_vec, test::*, *};
        use std::str::FromStr;

        #[test]
        fn player_can_discard_own_cards_to_the_crib() {
            let discarding = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let entity = Game::from(State::Discarding(discarding.clone()));
            let game_id = entity.id.clone();
            let discards = vec![card!("AH"), card!("2H")];
            let command = DiscardCardsToCrib::new(entity.id, PLAYER0, discards.clone());

            let test = GameTestFramework::new(entity.id, entity).when(command);
            let event = test.event();

            let Event::CardsDiscardedToCrib {
                game_id: actual_game_id,
                player: actual_player,
                discards: actual_discards,
            } = event
            else {
                panic!("unexpected event")
            };

            assert_eq!(actual_game_id, &game_id);
            assert_eq!(actual_player, &PLAYER0);
            assert_eq!(actual_discards, &discards);

            let State::Discarding(actual_discarding) = test.entity().state() else {
                panic!("unexpected state")
            };

            assert_eq!(actual_discarding.scoreboard(), discarding.scoreboard());
            assert_eq!(actual_discarding.dealer(), discarding.dealer());

            assert!(actual_discarding.hand(PLAYER0).contains_none(&discards));
            assert!(actual_discarding.crib().contains_all(&discards));
            assert_eq!(actual_discarding.hand(PLAYER1), discarding.hand(PLAYER1));
            assert_eq!(actual_discarding.deck(), discarding.deck());
        }

        #[test]
        fn player_cannot_discard_other_than_two_held_cards_to_the_crib() {
            let discarding = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            for discards in vec![
                vec![card!("AH"), card!("2H"), card!("3H")],
                vec![card!("AH")],
            ] {
                let entity = Game::from(State::Discarding(discarding.clone()));
                let command = DiscardCardsToCrib::new(entity.id, PLAYER0, discards.clone());

                let expected_discards = format_vec(&discards);

                GameTestFramework::new(entity.id, entity)
                    .when(command)
                    .expect_error(Error::InvalidDiscards(expected_discards));
            }
        }

        #[test]
        fn player_cannot_discard_unowned_cards_to_the_crib() {
            let discarding = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let entity = Game::from(State::Discarding(discarding.clone()));
            let discards = vec![card!("AC"), card!("2C")];
            let command = DiscardCardsToCrib::new(entity.id, PLAYER0, discards.clone());

            let expected_discards = format_vec(&discards);

            GameTestFramework::new(entity.id, entity)
                .when(command)
                .expect_error(Error::InvalidDiscards(expected_discards));
        }

        #[test]
        fn player_cannot_discard_if_already_discarded() {
            let discarding = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let entity = Game::from(State::Discarding(discarding.clone()));

            let preconditions = vec![Event::CardsDiscardedToCrib {
                game_id: entity.id,
                player: PLAYER0,
                discards: vec![card!("AH"), card!("2H")],
            }];

            let discards = vec![card!("3H"), card!("4H")];
            let command = DiscardCardsToCrib::new(entity.id, PLAYER0, discards.clone());

            let expected_discards = format_vec(&discards);

            GameTestFramework::new(entity.id, entity)
                .given(preconditions)
                .when(command)
                .expect_error(Error::InvalidDiscards(expected_discards));
        }
    }

    /// ## Before the Play
    ///
    /// After the crib is laid away, the non-dealer cuts the pack. The dealer turns up the top card
    /// of the lower packet and places it face up on top of the pack. This card is the "starter." If
    /// the starter is a jack, it is called "His Heels," and the dealer pegs (scores) 2 points at
    /// once. The starter is not used in the play phase of Cribbage , but is used later for making
    /// various card combinations that score points.
    mod before_the_play {
        use super::*;
        use crate::{
            Cut, Dealer, HasFace, Points, Pone, Scoreboard, State,
            constants::*,
            test::{GameBuilder, GameTestFramework},
        };

        fn after_discards_common_tests() -> (Scoreboard, Scoreboard, Cut, Dealer, Pone) {
            let discarding0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                .into_discarding();

            let scoreboard0 = discarding0.scoreboard().clone();
            let dealer0 = discarding0.dealer();
            let pone0 = discarding0.pone();

            let mut player_hand0 = discarding0.hand(PLAYER0).clone();
            let player_discards = player_hand0.take(2);

            let mut opponent_hand0 = discarding0.hand(PLAYER1).clone();
            let opponent_discards = opponent_hand0.take(2);

            let deck0 = discarding0.deck().clone();

            let entity = Game::from(State::Discarding(discarding0.clone()));
            let game_id = entity.id;

            let preconditions = vec![
                Event::CardsDiscardedToCrib {
                    game_id,
                    player: PLAYER0,
                    discards: player_discards.clone(),
                },
                Event::CardsDiscardedToCrib {
                    game_id,
                    player: PLAYER1,
                    discards: opponent_discards.clone(),
                },
            ];

            let test = GameTestFramework::new(entity.id, entity).given(preconditions);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state");
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer();
            let player_hand1 = playing1.hand(PLAYER0);
            let opponent_hand1 = playing1.hand(PLAYER1);
            let play_state1 = playing1.play_state();
            let cut1 = playing1.cut();
            let crib1 = playing1.crib();

            assert_eq!(dealer1, dealer0);

            assert!(player_hand1.contains_none(&player_discards));
            assert!(crib1.contains_all(&player_discards));

            assert!(opponent_hand1.contains_none(&opponent_discards));
            assert!(crib1.contains_all(&opponent_discards));
            assert!(deck0.contains(cut1));

            assert_eq!(crib1.len(), CARDS_REQUIRED_IN_CRIB);
            assert_eq!(
                play_state1.legal_plays(pone0.player()).as_slice(),
                playing1.hand(pone0.player()).as_ref()
            );
            assert_eq!(
                play_state1.legal_plays(dealer0.player()).as_slice(),
                playing1.hand(dealer0.player()).as_ref()
            );
            assert_eq!(play_state1.pass_count(), 0);
            assert_eq!(play_state1.current_plays(), []);
            assert_eq!(play_state1.previous_plays(), []);

            (scoreboard0, scoreboard1.clone(), cut1, *dealer1, *pone0)
        }

        #[test]
        fn start_the_play_after_discards() {
            let (scoreboard0, scoreboard1, cut, dealer, pone) = after_discards_common_tests();
            if cut.face().is_jack() {
                assert_eq!(
                    *scoreboard0.pegging(dealer.player()) + Points::from(2),
                    *scoreboard1.pegging(dealer.player())
                );
                assert_eq!(
                    *scoreboard0.pegging(pone.player()),
                    *scoreboard1.pegging(pone.player())
                );
            } else {
                assert_eq!(scoreboard0, scoreboard1)
            }
        }

        #[test]
        fn score_his_heels_when_jack_cut_after_discards() {
            loop {
                let (scoreboard0, scoreboard1, cut, dealer, pone) = after_discards_common_tests();
                if cut.face().is_jack() {
                    assert_eq!(
                        *scoreboard0.pegging(dealer.player()) + Points::from(2),
                        *scoreboard1.pegging(dealer.player())
                    );
                    assert_eq!(
                        *scoreboard0.pegging(pone.player()),
                        *scoreboard1.pegging(pone.player())
                    );
                    break;
                }
            }
        }

        #[test]
        fn finish_game_when_jack_cut_after_discards() {
            loop {
                let discarding0 = GameBuilder::default()
                    .with_peggings(120, 120)
                    .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
                    .into_discarding();

                let mut player_hand0 = discarding0.hand(PLAYER0).clone();
                let player_discards = player_hand0.take(2);

                let mut opponent_hand0 = discarding0.hand(PLAYER1).clone();
                let opponent_discards = opponent_hand0.take(2);

                let entity = Game::from(State::Discarding(discarding0.clone()));
                let game_id = entity.id;

                let preconditions = vec![
                    Event::CardsDiscardedToCrib {
                        game_id,
                        player: PLAYER0,
                        discards: player_discards.clone(),
                    },
                    Event::CardsDiscardedToCrib {
                        game_id,
                        player: PLAYER1,
                        discards: opponent_discards.clone(),
                    },
                ];

                let test = GameTestFramework::new(entity.id, entity).given(preconditions);

                if let State::Playing(playing1) = &test.entity().state {
                    assert!(!playing1.cut().face().is_jack())
                } else if let State::Finished(finished1) = &test.entity().state {
                    let scoreboard1 = finished1.scoreboard();
                    assert_eq!(scoreboard1.winner(), Some(PLAYER0));
                    assert_eq!(scoreboard1.pegging(PLAYER0).points(), Points::from(122));
                    assert_eq!(scoreboard1.pegging(PLAYER1).points(), Points::from(120));
                    break;
                } else {
                    panic!("unexpected state")
                };
            }
        }
    }

    /// ## The Play
    ///
    /// After the starter is turned, the non-dealer lays one of his cards face up on the table. The
    /// dealer similarly exposes a card, then non-dealer again, and so on - the hands are exposed
    /// card by card, alternately except for a "Go," (Pass) as noted below. Each player keeps his
    /// cards separate from those of his opponent.
    ///
    /// As each person plays, he announces a running total of pips reached by the addition of the
    /// last card to all those previously2 played. (Example: The non-dealer begins with a four,
    /// saying "Four." The dealer plays a nine, saying "Thirteen".) The kings, queens and jacks
    /// count 10 each; every other card counts its pip value (the ace counts one).
    #[coverage(off)]
    mod the_play {
        use crate::{
            Card, Error, Event, Game, GameBuilder, GameTestFramework, Hand, PLAYER0, PLAYER1, Play,
            PlayCard, Points, State, card, hand,
        };
        use std::str::FromStr;

        #[test]
        fn accept_valid_play() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .into_playing(1);
            let pone0 = playing0.pone().player();
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let dealer_hand0 = playing0.hand(dealer0).clone();
            let dealer_score0 = scoreboard0.pegging(dealer0);
            let pone_score0 = scoreboard0.pegging(pone0);
            let play_state0 = playing0.play_state();
            let cut0 = playing0.cut();
            let crib0 = playing0.crib().clone();

            assert_eq!(
                &play_state0.legal_plays(dealer0),
                &playing0.hand(dealer0).as_ref()
            );
            assert_eq!(&play_state0.legal_plays(pone0), &hand!("4S").as_ref());

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("4S"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state");
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();
            let dealer_hand1 = playing1.hand(dealer1).clone();
            let pone_hand1 = playing1.hand(pone1).clone();
            let play_state1 = playing1.play_state();
            let cut1 = playing1.cut();
            let crib1 = playing1.crib();
            let dealer_score1 = scoreboard1.pegging(dealer1);
            let pone_score1 = scoreboard1.pegging(pone1);

            assert_eq!(dealer_score1, dealer_score0);
            assert_eq!(pone_score1, pone_score0);
            assert_eq!(dealer1, dealer0);
            assert_eq!(dealer_hand1, dealer_hand0);
            assert_eq!(pone_hand1, Hand::default());
            assert_eq!(play_state1.next_to_play(), dealer1);
            assert_eq!(play_state1.legal_plays(dealer1), dealer_hand1.as_ref());
            assert_eq!(cut1, cut0);
            assert_eq!(crib1, &crib0);
        }

        #[test]
        fn accept_valid_play_after_opponent_passed() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4SAS")
                .with_cut("AC")
                .with_current_plays(&[(1, "TC"), (0, "TD"), (0, "5C")])
                .with_pass()
                .into_playing(1);
            let pone0 = playing0.pone().player();
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let dealer_hand0 = playing0.hand(dealer0).clone();
            let play_state0 = playing0.play_state();
            let cut0 = playing0.cut();
            let crib0 = playing0.crib().clone();
            let dealer_score0 = scoreboard0.pegging(dealer0);
            let pone_score0 = scoreboard0.pegging(pone0);

            assert_eq!(play_state0.legal_plays(dealer0), Hand::default().as_ref());
            assert_eq!(play_state0.legal_plays(pone0), hand!("4SAS").as_ref());

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("4S"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state");
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();
            let dealer_hand1 = playing1.hand(dealer1).clone();
            let pone_hand1 = playing1.hand(pone1).clone();
            let play_state1 = playing1.play_state();
            let cut1 = playing1.cut();
            let crib1 = playing1.crib();
            let dealer_score1 = scoreboard1.pegging(dealer1);
            let pone_score1 = scoreboard1.pegging(pone1);

            assert_eq!(dealer_score1, dealer_score0);
            assert_eq!(pone_score1, pone_score0);
            assert_eq!(dealer1, dealer0);
            assert_eq!(dealer_hand1, dealer_hand0);
            assert_eq!(pone_hand1, hand!("AS"));
            assert_eq!(play_state1.next_to_play(), pone1);
            assert_eq!(play_state1.legal_plays(dealer1), Hand::default().as_ref());
            assert_eq!(cut1, cut0);
            assert_eq!(crib1, &crib0);
        }

        #[test]
        fn cannot_play_when_unheld_card() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .into_playing(1);

            let card = card!("9S");
            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card);
            GameTestFramework::new(entity.id, entity)
                .when(command)
                .expect_error(Error::InvalidPlay(card));
        }

        #[test]
        fn cannot_play_when_not_their_turn() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .into_playing(1);
            let card = card!("9S");

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER0, card);
            GameTestFramework::new(entity.id, entity)
                .when(command)
                .expect_error(Error::NotPlayersTurn(PLAYER0));
        }

        #[test]
        fn cannot_play_when_play_exceeds_target() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9S", "4S")
                .with_cut("AS")
                .with_current_plays(&[(0, "KH"), (0, "KC"), (0, "KD")])
                .into_playing(1);

            let card = card!("4S");
            let play_state0 = playing0.play_state();
            assert_eq!(play_state0.legal_plays(PLAYER1), hand!("").as_ref());

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card);
            GameTestFramework::new(entity.id, entity)
                .when(command)
                .expect_error(Error::InvalidPlay(card));
        }

        #[test]
        fn score_play_when_target_not_reached_mid_play() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("5S", "5H")
                .with_cut("AS")
                .with_current_plays(&[(0, "TH")])
                .into_playing(1);
            let pone0 = playing0.pone().player();
            let scoreboard0 = playing0.scoreboard().clone();
            let score0_pone = scoreboard0.pegging(pone0);

            let card = card!("5H");
            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card);

            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state");
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let play_state1 = playing1.play_state();
            let score1_pone = scoreboard1.pegging(pone0);

            assert_eq!(*score1_pone, score0_pone.clone() + Points::from(2));
            assert_eq!(play_state1.next_to_play(), dealer1);
        }

        #[test]
        fn score_play_when_target_not_reached_end_play() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("QS", "2H")
                .with_cut("QC")
                .with_current_plays(&[(0, "JH"), (0, "QH")])
                .with_previous_plays(&[(0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
                .into_playing(1);

            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();
            let score0_dealer = scoreboard0.pegging(dealer0);
            let score0_pone = scoreboard0.pegging(pone0);

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("2H"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let play_state1 = playing1.play_state();
            let score1_pone = scoreboard1.pegging(pone0);
            let score1_dealer = scoreboard1.pegging(dealer1);

            assert_eq!(*score1_pone, score0_pone.clone() + Points::from(1));
            assert_eq!(score1_dealer, score0_dealer);
            assert_eq!(play_state1.next_to_play(), dealer1);
            // TODO: assert score_history...
        }

        #[test]
        fn score_play_when_target_not_reached_finished() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 120)
                .with_hands("AH", "5H")
                .with_cut("QC")
                .with_current_plays(&[(0, "JH")])
                .with_previous_plays(&[(0, "9H"), (0, "7C"), (1, "6S"), (1, "2S"), (1, "KS")])
                .into_playing(1);

            let scoresboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();
            let score0_pone = scoresboard0.pegging(pone0);
            let hand0_pone = playing0.hand(pone0).clone();
            let hand0_dealer = playing0.hand(dealer0).clone();
            let crib0 = playing0.crib().clone();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("5H"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Finished(finished1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let winner1 = finished1.winner();
            let scoreboard1 = finished1.scoreboard();
            let score1_pone = scoreboard1.pegging(pone0);
            let hand1_pone = finished1.hand(pone0).clone();
            let hand1_dealer = finished1.hand(dealer0);
            let crib1 = finished1.crib();

            assert_eq!(winner1, pone0);
            assert_eq!(*score1_pone, score0_pone.clone() + Points::from(2));
            assert!(!hand1_pone.contains_all(hand0_pone.as_ref()));
            assert!(!hand1_pone.contains(card!("5H")));
            assert!(hand1_dealer.contains_all(hand0_dealer.as_ref()));
            assert!(hand0_dealer.contains_all(hand1_dealer.as_ref()));
            assert!(crib1.contains_all(crib0.as_ref()));
            assert!(crib0.contains_all(crib1.as_ref()));
        }

        #[test]
        fn score_play_when_target_reached_mid_play() {
            let card = card!("AH");
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("9H", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "2S"), (1, "QS"), (1, "6S")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();
            let play_state0 = playing0.play_state().clone();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card);
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone_hand1 = playing1.hand(PLAYER1);
            let play_state1 = playing1.play_state();
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(2)
            );
            assert_eq!(dealer1, dealer0);
            assert!(!pone_hand1.contains(card));
            assert_eq!(play_state1.next_to_play(), dealer0);
            assert!(play_state1.current_plays().is_empty());
            for p in play_state0.current_plays().into_iter() {
                assert!(play_state1.previous_plays().contains(&p))
            }
        }

        #[test]
        fn score_play_when_target_reached_end_play() {
            let card = card!("AH");
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_hands("QC", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "2S"), (1, "QS"), (1, "6S")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();
            let cut0 = playing0.cut();
            let crib0 = playing0.crib().clone();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card);
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let play_state1 = playing1.play_state();
            let cut1 = playing1.cut();
            let crib1 = playing1.crib();

            assert_eq!(dealer1, dealer0);
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(2)
            );
            assert_eq!(play_state1.next_to_play(), dealer1);
            assert_eq!(cut1, cut0);
            assert_eq!(crib1, &crib0);
        }

        #[test]
        fn score_play_when_target_reached_finished() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 120)
                .with_hands("QC", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "TH"), (1, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "9H"), (1, "5S"), (0, "6S")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("AH"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Finished(finished1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let winner1 = finished1.winner();
            let scoreboard1 = finished1.scoreboard();

            assert_eq!(winner1, pone0);
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(2)
            );
            assert_eq!(scoreboard1.pegging(dealer0), scoreboard0.pegging(dealer0));
        }

        #[test]
        fn score_play_when_plays_finished_and_game_not_finished() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 60)
                .with_hands("", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "8H"), (1, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "9H"), (0, "4S"), (1, "5S"), (0, "6S")])
                .into_playing(1);
            let pone0 = playing0.pone().player();
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("AH"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::ScoringPone(scoring1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = scoring1.scoreboard();
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(1)
            );
            assert_eq!(scoreboard1.pegging(dealer0), scoreboard0.pegging(dealer0));
        }

        #[test]
        fn score_play_when_plays_finished_and_game_finished() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 120)
                .with_hands("", "AH")
                .with_cut("KC")
                .with_current_plays(&[(0, "8H"), (1, "JH"), (0, "QH")])
                .with_previous_plays(&[(1, "9H"), (0, "4S"), (1, "5S"), (0, "6S")])
                .into_playing(1);
            let pone0 = playing0.pone().player();
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("AH"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Finished(finished1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let winner1 = finished1.winner();
            let scoreboard1 = finished1.scoreboard();

            assert_eq!(winner1, pone0);
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(1)
            );
            assert_eq!(scoreboard1.pegging(dealer0), scoreboard0.pegging(dealer0));
        }

        #[test]
        fn swap_player_after_pone_play() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D9C", "4S5STHJH")
                .into_playing(1);
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, PLAYER1, card!("4S"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();
            let play_state1 = playing1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), dealer1);
        }

        #[test]
        fn swap_player_after_dealer_play() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D9C", "5STHJH")
                .with_current_plays(&[(1, "4S")])
                .into_playing(0);
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, dealer0, card!("9C"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();
            let play_state1 = playing1.play_state();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), pone0);
        }

        #[test]
        fn reset_play_after_exact_target_reached() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("7H8H8D", "5STH")
                .with_current_plays(&[(1, "JH"), (0, "9C"), (1, "4S")])
                .into_playing(0);
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();
            let play_state0 = playing0.play_state().clone();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, dealer0, card!("8H"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();
            let play_state1 = playing1.play_state();

            let last_play = Play::new(dealer0, card!("8H"));
            let mut expected_current_plays = play_state0.current_plays();
            expected_current_plays.push(last_play);

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(play_state1.next_to_play(), pone0);
            assert_eq!(play_state1.previous_plays(), expected_current_plays);
            assert!(play_state1.current_plays().is_empty());
            assert_eq!(play_state1.pass_count(), 0);
        }

        #[test]
        fn score_play_points_for_fifteens() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "8D")
                .with_current_plays(&[(0, "7D")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, pone0, card!("8D"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scoreboard1.pegging(dealer1), scoreboard0.pegging(dealer0));
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(2)
            );
        }

        #[test]
        fn score_play_points_for_pair() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "8D")
                .with_current_plays(&[(0, "8S")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, pone0, card!("8D"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scoreboard1.pegging(dealer1), scoreboard0.pegging(dealer0));
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(2)
            );
        }

        #[test]
        fn score_play_points_for_triplet() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "8DAH")
                .with_current_plays(&[(1, "8C"), (0, "8S")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, pone0, card!("8D"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scoreboard1.pegging(dealer1), scoreboard0.pegging(dealer0));
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(6)
            );
        }

        #[test]
        fn score_play_points_for_quartet() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("KH", "7DAH")
                .with_current_plays(&[(1, "7C"), (0, "7S"), (0, "7H")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, pone0, card!("7D"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scoreboard1.pegging(dealer1), scoreboard0.pegging(dealer0));
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(12)
            );
        }

        #[test]
        fn score_play_points_for_run() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AC")
                .with_hands("KH", "AS")
                .with_current_plays(&[(1, "2D"), (0, "3H")])
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let command = PlayCard::new(entity.id, pone0, card!("AS"));
            let test = GameTestFramework::new(entity.id, entity).when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(scoreboard1.pegging(dealer1), scoreboard0.pegging(dealer0));
            assert_eq!(
                *scoreboard1.pegging(pone0),
                scoreboard0.pegging(pone0).clone() + Points::from(3)
            );
        }

        #[test]
        fn score_play_points_for_run_edge_case_1() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("5H7H6H", "AH8S7S")
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let game_id = entity.id;
            let command = PlayCard::new(game_id, dealer0, card!("6H"));
            let test = GameTestFramework::new(game_id, entity)
                .given(vec![
                    Event::CardPlayed {
                        game_id: game_id,
                        player: pone0,
                        card: card!("8S"),
                    },
                    Event::CardPlayed {
                        game_id: game_id,
                        player: dealer0,
                        card: card!("7H"),
                    },
                    Event::CardPlayed {
                        game_id: game_id,
                        player: pone0,
                        card: card!("7S"),
                    },
                ])
                .when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(
                *scoreboard1.pegging(dealer1),
                scoreboard0.pegging(dealer0).clone() + Points::from(2)
            );
            assert_eq!(
                *scoreboard1.pegging(pone1),
                scoreboard0.pegging(pone0).clone() + Points::from(2)
            );
        }

        #[test]
        fn score_play_points_for_run_edge_case_2() {
            let playing0 = GameBuilder::default()
                .with_peggings(0, 0)
                .with_cut("AS")
                .with_hands("5H7H6H", "AH9S8S")
                .into_playing(1);
            let scoreboard0 = playing0.scoreboard().clone();
            let dealer0 = playing0.dealer().player();
            let pone0 = playing0.pone().player();

            let entity = Game::from(State::Playing(playing0));
            let game_id = entity.id;
            let command = PlayCard::new(game_id, dealer0, card!("7H"));
            let test = GameTestFramework::new(game_id, entity)
                .given(vec![
                    Event::CardPlayed {
                        game_id: game_id,
                        player: pone0,
                        card: card!("9S"),
                    },
                    Event::CardPlayed {
                        game_id: game_id,
                        player: dealer0,
                        card: card!("6H"),
                    },
                    Event::CardPlayed {
                        game_id: game_id,
                        player: pone0,
                        card: card!("8S"),
                    },
                ])
                .when(command);

            let State::Playing(playing1) = &test.entity().state else {
                panic!("unexpected state")
            };

            let scoreboard1 = playing1.scoreboard();
            let dealer1 = playing1.dealer().player();
            let pone1 = playing1.pone().player();

            assert_eq!(dealer1, dealer0);
            assert_eq!(pone1, pone0);
            assert_eq!(
                *scoreboard1.pegging(dealer1),
                scoreboard0.pegging(dealer0).clone() + Points::from(2) + Points::from(4)
            );
            assert_eq!(scoreboard1.pegging(pone1), scoreboard0.pegging(pone0));
        }
    }

    // /// ## The Go
    // ///
    // /// During play, the running total of cards may never be carried beyond 31. If a player cannot
    // /// add another card without exceeding 31, he or she says "Go" and the opponent pegs 1. After
    // /// gaining the Go, the opponent must first lay down any additional cards he can without
    // /// exceeding 31. Besides the point for Go, he may then score any additional points that can be
    // /// made through pairs and runs (described later). If a player reaches exactly 31, he pegs two
    // /// instead of one for Go.
    // ///
    // /// The player who called Go leads for the next series of plays, with the count starting at
    // /// zero. The lead may not be combined with any cards previously played to form a scoring
    // /// combination; the Go has interrupted the sequence.
    // ///
    // /// The person who plays the last card pegs one for Go, plus one extra if the card brings the
    // /// count to exactly 31. The dealer is sure to peg at least one point in every hand, for he will
    // /// have a Go on the last card if not earlier.
    // // #[coverage(off)]
    // // mod the_go {
    // //     use super::*;

    // //     #[test]
    // //     fn accept_pass_when_pone_has_no_valid_card() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AS")
    // //             .with_hands("AH", "KH")
    // //             .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "QH")])
    // //             .into_playing(1);
    // //         let scoreboard0 = game0.scoreboard().clone();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();
    // //         let hands0 = game0.hands().clone();
    // //         let play_state0 = game0.play_state().clone();

    // //         let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
    // //             panic!("unexpected result")
    // //         };

    // //         let scoreboard1 = game1.scoreboard();
    // //         let dealer1 = game1.dealer().player();
    // //         let hands1 = game1.hands();
    // //         let play_state1 = game1.play_state();

    // //         assert_eq!(scoreboard1, &scoreboard0);
    // //         assert_eq!(dealer1, dealer0);
    // //         assert_eq!(hands1, &hands0);
    // //         assert_eq!(play_state1.next_to_play(), dealer0);
    // //         assert_eq!(play_state1.pass_count(), 1);
    // //         assert_eq!(play_state1.current_plays(), play_state0.current_plays());
    // //         assert_eq!(play_state1.previous_plays(), play_state0.previous_plays());
    // //     }

    // //     #[test]
    // //     fn accept_pass_when_dealer_has_no_valid_card() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AS")
    // //             .with_hands("KH", "KS")
    // //             .with_current_plays(&[(0, "TH"), (0, "JH"), (1, "QH")])
    // //             .into_playing(1);
    // //         let scoreboard0 = game0.scoreboard().clone();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();
    // //         let hands0 = game0.hands().clone();
    // //         let play_state0 = game0.play_state().clone();

    // //         let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let PassResult::Playing(game1) = game1.pass(dealer0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard();
    // //         let dealer1 = game1.dealer().player();
    // //         let hands1 = game1.hands();
    // //         let play_state1 = game1.play_state();

    // //         assert_eq!(scoreboard1.score(pone0), scoreboard0.score(pone0));
    // //         assert_eq!(
    // //             *scoreboard1.score(dealer1),
    // //             scoreboard0.score(dealer0).clone() + Points::from(1)
    // //         );
    // //         assert_eq!(dealer1, dealer0);
    // //         assert_eq!(hands1, &hands0);
    // //         assert_eq!(play_state1.next_to_play(), pone0.into());
    // //         assert_eq!(play_state1.pass_count(), 0);
    // //         assert!(play_state1.current_plays().is_empty());
    // //         for p in play_state0.current_plays().into_iter() {
    // //             assert!(play_state1.previous_plays().contains(&p))
    // //         }
    // //     }

    // //     #[test]
    // //     fn cannot_pass_when_valid_card_held() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AC")
    // //             .with_hands("AH", "AS")
    // //             .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "8H")])
    // //             .into_playing(1);
    // //         let pone0 = game0.pone().player();

    // //         let error = game0.pass(pone0).expect_err("invalid pass");
    // //         assert_eq!(error, GameError::CannotPass);
    // //     }

    // //     #[test]
    // //     fn cannot_pass_when_not_turn() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AC")
    // //             .with_hands("AH", "AS")
    // //             .with_current_plays(&[(0, "TH"), (0, "JH"), (0, "8H")])
    // //             .into_playing(1);
    // //         let dealer0 = game0.dealer().player();

    // //         let error = game0.pass(dealer0).expect_err("invalid pass");
    // //         assert_eq!(error, GameError::PlayOrPassNotPermittedByPlayer);
    // //     }

    // //     #[test]
    // //     fn score_pass_when_both_players_passed_playing() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AS")
    // //             .with_hands("KH", "KS")
    // //             .with_current_plays(&[(0, "TH"), (0, "JH"), (1, "QH")])
    // //             .into_playing(1);
    // //         let scoreboard0 = game0.scoreboard().clone();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let PassResult::Playing(game1) = game1.pass(dealer0).expect("valid pass") else {
    // //             panic!("unexpectd state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard();
    // //         let dealer1 = game1.dealer().player();
    // //         let pone1 = game1.pone().player();

    // //         assert_eq!(scoreboard1.score(pone1), scoreboard0.score(pone0));
    // //         assert_eq!(
    // //             *scoreboard1.score(dealer1),
    // //             scoreboard0.score(dealer0).clone() + Points::from(1)
    // //         );
    // //     }

    // //     #[test]
    // //     fn score_pass_when_both_players_passed_finished() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(120, 0)
    // //             .with_cut("AS")
    // //             .with_hands("KH", "KS")
    // //             .with_current_plays(&[(0, "TH"), (0, "JH"), (1, "QH")])
    // //             .into_playing(1);
    // //         let scoreboard0 = game0.scoreboard().clone();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let PassResult::Finished(game1) = game1.pass(dealer0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let winner1 = game1.winner();
    // //         let scoreboard1 = game1.scoreboard();

    // //         assert_eq!(winner1, dealer0.into());
    // //         assert_eq!(scoreboard1.score(pone0), scoreboard0.score(pone0));
    // //         assert_eq!(
    // //             *scoreboard1.score(dealer0),
    // //             scoreboard0.score(dealer0).clone() + Points::from(1)
    // //         );
    // //     }

    // //     #[test]
    // //     fn swap_player_after_pone_pass() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AS")
    // //             .with_hands("8H8D", "5SJH")
    // //             .with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
    // //             .into_playing(1);
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
    // //             panic!("unexpected event")
    // //         };

    // //         let dealer1 = game1.dealer().player();
    // //         let pone1 = game1.pone().player();
    // //         let play_state1 = game1.play_state();

    // //         assert_eq!(dealer1, dealer0);
    // //         assert_eq!(pone1, pone0);
    // //         assert_eq!(play_state1.next_to_play(), dealer1.into());
    // //     }

    // //     #[test]
    // //     fn swap_player_after_dealer_pass() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AS")
    // //             .with_hands("7H8H8D", "4S5S")
    // //             .with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
    // //             .into_playing(0);
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let PassResult::Playing(game1) = game0.pass(dealer0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let dealer1 = game1.dealer().player();
    // //         let pone1 = game1.pone().player();
    // //         let play_state1 = game1.play_state();

    // //         assert_eq!(dealer1, dealer0);
    // //         assert_eq!(pone1, pone0);
    // //         assert_eq!(play_state1.next_to_play(), pone0.into());
    // //     }

    // //     #[test]
    // //     fn reset_play_after_pone_then_dealer_pass() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AS")
    // //             .with_hands("8H8D", "5SJH")
    // //             .with_current_plays(&[(1, "4S"), (0, "9C"), (1, "TH"), (0, "7H")])
    // //             .into_playing(1);
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();
    // //         let play_state0 = game0.play_state().clone();

    // //         let PassResult::Playing(game1) = game0.pass(pone0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let PassResult::Playing(game1) = game1.pass(dealer0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let dealer1 = game1.dealer().player();
    // //         let pone1 = game1.pone().player();
    // //         let play_state1 = game1.play_state();

    // //         assert_eq!(dealer1, dealer0);
    // //         assert_eq!(pone1, pone0);
    // //         assert_eq!(play_state1.next_to_play(), pone1.into());
    // //         assert_eq!(play_state1.previous_plays(), play_state0.current_plays());
    // //         assert!(play_state1.current_plays().is_empty());
    // //         assert_eq!(play_state1.pass_count(), 0);
    // //     }

    // //     #[test]
    // //     fn reset_play_after_after_dealer_then_pone_pass() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("AS")
    // //             .with_hands("7H8H8D", "4S5S")
    // //             .with_current_plays(&[(1, "JH"), (0, "9C"), (1, "TH")])
    // //             .into_playing(0);
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();
    // //         let play_state0 = game0.play_state().clone();

    // //         let PassResult::Playing(game1) = game0.pass(dealer0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let PassResult::Playing(game1) = game1.pass(pone0).expect("valid pass") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let dealer1 = game1.dealer().player();
    // //         let pone1 = game1.pone().player();
    // //         let play_state1 = game1.play_state();

    // //         assert_eq!(dealer1, dealer0);
    // //         assert_eq!(pone1, pone0);
    // //         assert_eq!(play_state1.next_to_play(), dealer1.into());
    // //         assert_eq!(play_state1.previous_plays(), play_state0.current_plays());
    // //         assert!(play_state1.current_plays().is_empty());
    // //         assert_eq!(play_state1.pass_count(), 0);
    // //     }
    // // }

    // /// ## Pegging
    // ///
    // /// The object in play is to score points by pegging. In addition to a Go, a player may score
    // /// for the following combinations:
    // ///
    // ///   - Fifteen: For adding a card that makes the total 15 Peg 2
    // ///   - Pair: For adding a card of the same rank as the card just played Peg 2 (Note that face
    // ///     cards pair only by actual rank: jack with jack, but not jack with queen.)
    // ///   - Triplet: For adding the third card of the same rank. Peg 6
    // ///   - Four: (also called "Double Pair" or "Double Pair Royal") For adding the fourth card of
    // ///     the same rank Peg 12
    // ///   - Run (Sequence): For adding a card that forms, with those just played:
    // ///     - For a sequence of three Peg 3
    // ///     - For a sequence of four. Peg 4
    // ///     - For a sequence of five. Peg 5
    // ///     - (Peg one point more for each extra card of a sequence. Note that runs are independent
    // ///       of suits, but go strictly by rank; to illustrate: 9, 10, J, or J, 9, 10 is a run but
    // ///       9, 10, Q is not)
    // ///
    // /// It is important to keep track of the order in which cards are played to determine whether
    // /// what looks like a sequence or a run has been interrupted by a "foreign card." Example:
    // /// Cards are played in this order: 8, 7, 7, 6. The dealer pegs 2 for 15, and the opponent
    // /// pegs 2 for pair, but the dealer cannot peg for run because of the extra seven (foreign
    // /// card) that has been played. Example: Cards are played in this order: 9, 6, 8, 7. The
    // /// dealer pegs 2 for fifteen when he plays the six and pegs 4 for run when he plays the seven
    // /// (the 6, 7, 8, 9 sequence). The cards were not played in sequential order, but they form a
    // /// true run with no foreign card.
    // // #[coverage(off)]
    // // mod pegging {
    // //     use super::*;
    // //     use crate::card;
    // //     use std::str::FromStr;

    // //     #[test]
    // //     fn should_score_fifteens() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("", "")
    // //             .with_current_plays(&[(0, "JD"), (0, "5H")])
    // //             .with_cut("AH")
    // //             .into_playing(1);
    // //         let play_state = game.play_state();
    // //         assert_eq!(
    // //             CurrentPlayScorer::new(play_state).score().points(),
    // //             Points::from(2)
    // //         )
    // //     }

    // //     #[test]
    // //     fn should_score_pairs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("", "")
    // //             .with_current_plays(&[(0, "JD"), (0, "AH"), (0, "AS")])
    // //             .with_cut("KH")
    // //             .into_playing(1);
    // //         let play_state = game.play_state();
    // //         assert_eq!(
    // //             CurrentPlayScorer::new(play_state).score().points(),
    // //             Points::from(2)
    // //         )
    // //     }

    // //     #[test]
    // //     fn should_score_royal_pairs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("", "")
    // //             .with_current_plays(&[(0, "AD"), (0, "AH"), (0, "AS")])
    // //             .with_cut("KH")
    // //             .into_playing(1);
    // //         let play_state = game.play_state();
    // //         assert_eq!(
    // //             CurrentPlayScorer::new(play_state).score().points(),
    // //             Points::from(6)
    // //         )
    // //     }

    // //     #[test]
    // //     fn should_score_double_royal_pairs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("", "")
    // //             .with_current_plays(&[(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
    // //             .with_cut("KH")
    // //             .into_playing(1);
    // //         let play_state = game.play_state();
    // //         assert_eq!(
    // //             CurrentPlayScorer::new(play_state).score().points(),
    // //             Points::from(12)
    // //         )
    // //     }

    // //     #[test]
    // //     fn should_score_runs() {
    // //         let current_plays = &[
    // //             (0, "2C"),
    // //             (0, "3C"),
    // //             (0, "4C"),
    // //             (0, "5C"),
    // //             (0, "6C"),
    // //             (0, "7C"),
    // //         ];
    // //         for len in 1..=current_plays.len() {
    // //             let current_plays = *current_plays;
    // //             let current_plays = current_plays.into_iter().take(len);
    // //             let current_plays = Vec::from_iter(current_plays);
    // //             let game = GameBuilder::default()
    // //                 .with_peggings(0, 0)
    // //                 .with_hands("KS", "KD")
    // //                 .with_current_plays(&current_plays)
    // //                 .with_cut("KH")
    // //                 .into_playing(1);
    // //             let play_state = game.play_state();
    // //             assert_eq!(
    // //                 CurrentPlayScorer::new(play_state).score().points(),
    // //                 Points::from(if len < 3 { 0 } else { len })
    // //             )
    // //         }
    // //     }

    // //     #[test]
    // //     fn should_score_runs_unordered() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("KS", "KD")
    // //             .with_current_plays(&[(0, "3S"), (0, "2C"), (0, "AS")])
    // //             .with_cut("KH")
    // //             .into_playing(1);
    // //         let play_state = game.play_state();
    // //         assert_eq!(
    // //             CurrentPlayScorer::new(play_state).score().points(),
    // //             Points::from(3)
    // //         )
    // //     }

    // //     #[test]
    // //     fn should_score_rules_example_flush() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("AH", "KD")
    // //             .with_cut("2H")
    // //             .with_current_plays(&[(1, "TH"), (0, "9H"), (1, "QH")])
    // //             .into_playing(0);
    // //         let dealer0 = game0.dealer().player();

    // //         let PlayResult::Playing(game1) = game0.play(dealer0, card!("AH")).expect("valid play")
    // //         else {
    // //             panic!("unexpected state")
    // //         };

    // //         let play_state1 = game1.play_state();
    // //         assert_eq!(
    // //             CurrentPlayScorer::new(play_state1).score().points(),
    // //             Points::from(0)
    // //         );
    // //     }

    // //     #[test]
    // //     fn should_score_when_target_not_reached() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("", "")
    // //             .with_current_plays(&[(0, "AC"), (0, "AD"), (0, "AH"), (0, "AS")])
    // //             .with_cut("KH")
    // //             .into_playing(1);
    // //         let play_state = game.play_state();
    // //         assert_eq!(
    // //             EndOfPlayScorer::new(play_state).score().points(),
    // //             Points::from(1)
    // //         );
    // //     }

    // //     #[test]
    // //     fn should_score_when_target_reached() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("", "")
    // //             .with_current_plays(&[(0, "KC"), (0, "KD"), (0, "KH"), (0, "AS")])
    // //             .with_cut("KS")
    // //             .into_playing(1);
    // //         let play_state = game.play_state();
    // //         assert_eq!(
    // //             EndOfPlayScorer::new(play_state).score().points(),
    // //             Points::from(2)
    // //         )
    // //     }
    // // }

    // /// ## Counting the Hands
    // ///
    // /// When play ends, the three hands are counted in order: non-dealer's hand (first), dealer's
    // /// hand (second), and then the crib (third). This order is important because, toward the end of
    // /// a game, the non-dealer may "count out" and win before the dealer has a chance to count, even
    // /// though the dealer's total would have exceeded that of the opponent. The starter is
    // /// considered to be a part of each hand, so that all hands in counting comprise five cards. The
    // /// basic scoring formations are as follows:
    // ///
    // /// Combinations counts
    // ///   - Fifteen. Each combination of cards that totals 15 2
    // ///   - Pair. Each pair of cards of the same rank 2
    // ///   - Run. Each combination of three or more 1 cards in sequence (for each card in the
    // ///     sequence)
    // ///   - Flush.
    // ///     - Four cards of the same suit in hand 4 (excluding the crib, and the starter)
    // ///     - Four cards in hand or crib of the same 5 suit as the starter. (There is no count for
    // ///       four-flush in the crib that is not of same suit as the starter)
    // ///   - His Nobs. Jack of the same suit as starter in hand or crib 1
    // // #[coverage(off)]
    // // mod counting_the_hands {
    // //     use super::*;
    // //     use crate::{card, crib, hand};
    // //     use std::str::FromStr;

    // //     #[test]
    // //     fn score_pone_hand_when_plays_finished() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("", "TH")
    // //             .with_cut("4H")
    // //             .with_previous_plays(&[
    // //                 (0, "7H"),
    // //                 (0, "8C"),
    // //                 (0, "AC"),
    // //                 (0, "2C"),
    // //                 (1, "JH"),
    // //                 (1, "KS"),
    // //                 (1, "5H"),
    // //             ])
    // //             .into_playing(1);
    // //         let scoreboard0 = game0.scoreboard().clone();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let PlayResult::Scoring(game1) = game0.play(pone0, card!("TH")).expect("valid play") else {
    // //             panic!("Unexpected state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard();
    // //         let dealer1 = game1.dealer().player();
    // //         let pone1 = game1.pone().player();

    // //         assert_eq!(dealer1, dealer0);
    // //         assert_eq!(pone1, pone0);
    // //         assert_eq!(scoreboard1.score(dealer1), scoreboard0.score(dealer0));
    // //         assert_eq!(
    // //             *scoreboard1.score(pone1),
    // //             scoreboard0.score(pone0).clone() + Points::from(1)
    // //         );
    // //         assert!(matches!(game1.state(), State::ScoringPone(_)));
    // //     }

    // //     #[test]
    // //     fn score_winning_pone_when_plays_finished() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 115)
    // //             .with_hands("", "TH")
    // //             .with_cut("4H")
    // //             .with_previous_plays(&[
    // //                 (0, "7H"),
    // //                 (0, "8C"),
    // //                 (0, "AC"),
    // //                 (0, "2C"),
    // //                 (1, "JH"),
    // //                 (1, "KS"),
    // //                 (1, "5H"),
    // //             ])
    // //             .into_playing(1);

    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let PlayResult::Scoring(game1) = game0.play(pone0, card!("TH")).expect("valid play") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard().clone();
    // //         println!("scoreboard1: {scoreboard1}");

    // //         let ScorePoneResult::Finished(game2) = game1.score_hand().expect("valid score_hand") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let winner2 = game2.winner();
    // //         let scoreboard2 = game2.scoreboard();

    // //         assert_eq!(winner2, pone0.into());
    // //         assert_eq!(scoreboard2.score(dealer0), scoreboard1.score(dealer0));
    // //         assert_eq!(
    // //             *scoreboard2.score(pone0),
    // //             scoreboard1.score(pone0).clone() + Points::from(7)
    // //         );
    // //     }

    // //     #[test]
    // //     fn score_dealer_after_pone_scored() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("4H")
    // //             .with_hands("7H8CAC2C", "JCKS5HTH")
    // //             .into_scoring_pone();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let ScorePoneResult::Scoring(game1) = game0.score_hand().expect("valid score_hand") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard().clone();
    // //         assert!(matches!(game1.state(), State::ScoringDealer(_)));

    // //         let ScoreDealerResult::Scoring(game2) = game1.score_hand().expect("valid score_hand")
    // //         else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard2 = game2.scoreboard();
    // //         let dealer2 = game2.dealer().player();
    // //         let pone2 = game2.pone().player();

    // //         assert_eq!(dealer0, dealer2);
    // //         assert_eq!(pone0, pone2);
    // //         assert_eq!(
    // //             *scoreboard2.score(dealer2),
    // //             scoreboard1.score(dealer0).clone() + Points::from(4)
    // //         );
    // //         assert_eq!(scoreboard2.score(pone2), scoreboard1.score(pone0));
    // //         assert!(matches!(game2.state(), State::ScoringCrib(_)));
    // //     }

    // //     #[test]
    // //     fn score_winning_dealer_after_pone_scored() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(117, 0)
    // //             .with_cut("4H")
    // //             .with_hands("7H8CAC2C", "JCKS5HTH")
    // //             .into_scoring_pone();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let ScorePoneResult::Scoring(game1) = game0.score_hand().expect("valid score_hand") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard().clone();

    // //         let ScoreDealerResult::Finished(game2) = game1.score_hand().expect("valid score_hand")
    // //         else {
    // //             panic!("unexpected state")
    // //         };

    // //         let winner2 = game2.winner();
    // //         let scoreboard2 = game2.scoreboard();

    // //         assert_eq!(winner2, dealer0.into());
    // //         assert_eq!(
    // //             *scoreboard2.score(winner2),
    // //             scoreboard1.score(dealer0).clone() + Points::from(4)
    // //         );
    // //         assert_eq!(scoreboard2.score(pone0), scoreboard1.score(pone0));
    // //     }

    // //     #[test]
    // //     fn score_crib_after_dealer_scored() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("4H")
    // //             .with_hands("7H8CAC2C", "JCKS5HTH")
    // //             .with_crib("AHADASTD")
    // //             .into_scoring_dealer();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let ScoreDealerResult::Scoring(game1) = game0.score_hand().expect("valid score_hand")
    // //         else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard().clone();

    // //         let ScoreCribResult::Discarding(game2) = game1.score_crib().expect("valid score_crib")
    // //         else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard2 = game2.scoreboard();
    // //         let dealer2 = game2.dealer().player();
    // //         let pone2 = game2.pone().player();

    // //         assert_eq!(pone2, dealer0);
    // //         assert_eq!(dealer2, pone0);
    // //         assert_eq!(
    // //             *scoreboard2.score(pone2),
    // //             scoreboard1.score(dealer0).clone() + Points::from(12)
    // //         );
    // //         assert_eq!(scoreboard2.score(dealer2), scoreboard1.score(pone0));
    // //     }

    // //     #[test]
    // //     fn redeal_after_crib_scored() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_cut("4H")
    // //             .with_hands("7H8CAC2C", "JCKS5HTH")
    // //             .with_crib("AHADASTD")
    // //             .into_scoring_crib();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let ScoreCribResult::Discarding(game1) = game0.score_crib().expect("valid score_crib")
    // //         else {
    // //             panic!("unexpected state")
    // //         };

    // //         let dealer1 = game1.dealer().player();
    // //         let pone1 = game1.pone().player();
    // //         let hands1 = game1.hands();
    // //         let crib1 = game1.crib();
    // //         let deck1 = game1.deck();

    // //         assert_eq!(dealer1, pone0);
    // //         assert_eq!(pone1, dealer0);
    // //         assert_eq!(hands1[dealer1].len(), 6);
    // //         assert_eq!(hands1[pone1].len(), 6);
    // //         assert!(crib1.is_empty());
    // //         assert_eq!(deck1.len(), 40);
    // //     }

    // //     #[test]
    // //     fn score_winning_crib_after_dealer_scored() {
    // //         let game0 = GameBuilder::default()
    // //             .with_peggings(110, 0)
    // //             .with_cut("4H")
    // //             .with_hands("7H8CAC2C", "JCKS5HTH")
    // //             .with_crib("AHADASTD")
    // //             .into_scoring_dealer();
    // //         let dealer0 = game0.dealer().player();
    // //         let pone0 = game0.pone().player();

    // //         let ScoreDealerResult::Scoring(game1) = game0.score_hand().expect("valid score_hand")
    // //         else {
    // //             panic!("unexpected state")
    // //         };

    // //         let scoreboard1 = game1.scoreboard().clone();

    // //         let ScoreCribResult::Finished(game2) = game1.score_crib().expect("valid score_crib") else {
    // //             panic!("unexpected state")
    // //         };

    // //         let winner2 = game2.winner();
    // //         let scoreboard2 = game2.scoreboard();

    // //         assert_eq!(winner2, dealer0.into());
    // //         assert_eq!(
    // //             *scoreboard2.score(winner2),
    // //             scoreboard1.score(dealer0).clone() + Points::from(12)
    // //         );
    // //         assert_eq!(scoreboard2.score(pone0), scoreboard1.score(pone0));
    // //     }

    // //     #[test]
    // //     fn hand_should_score_fifteens() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("7H8CAC2C"), card!("4H"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(4)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("THJCKS5H"), card!("4H"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(6)
    // //         );
    // //     }

    // //     #[test]
    // //     fn hand_should_score_pairs() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("2H4C5C2C"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(2)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("TCASADTH"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn hand_should_score_royal_pairs() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("2H2D5C2C"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(6)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("TCASADTH"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn hand_should_score_double_royal_pairs() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("2H2C2D2S"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(12)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("TCASADTH"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn hand_should_score_runs() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("JDQCKC2C"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(3)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("3C3S2D5H"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn hand_should_score_flushes() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("2H4H6H8H"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(5)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("2D4D6D8D"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(4)
    // //         );
    // //     }

    // //     #[test]
    // //     fn hand_should_score_his_heels() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("2D4H6HJH"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(1)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("2H4D6DJD"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(0)
    // //         );
    // //     }

    // //     #[test]
    // //     fn crib_should_score_fifteens() {
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("7H8CAC2C"), card!("4H"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(4)
    // //         );
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("THJCKS5H"), card!("4H"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(6)
    // //         );
    // //     }

    // //     #[test]
    // //     fn crib_should_score_pairs() {
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("2H4C5C2C"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(2)
    // //         );
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("TCASADTH"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn crib_should_score_royal_pairs() {
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("2H2D5C2C"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(6)
    // //         );
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("TCASADTH"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn crib_should_score_double_royal_pairs() {
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("2H2C2D2S"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(12)
    // //         );
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("TCASADTH"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn crib_should_score_runs() {
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("JDQCKC2C"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(3)
    // //         );
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("3C3S2D5H"), card!("AH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(8)
    // //         );
    // //     }

    // //     #[test]
    // //     fn crib_should_score_flushes() {
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("2H4H6H8H"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(5)
    // //         );
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("2D4D6D8D"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(0)
    // //         );
    // //     }

    // //     #[test]
    // //     fn crib_should_score_his_heels() {
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("2D4H6HJH"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(1)
    // //         );
    // //         assert_eq!(
    // //             CribScorer::new(&crib!("2H4D6DJD"), card!("TH"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(0)
    // //         );
    // //     }
    // // }

    // /// ### Combinations
    // ///
    // /// In the above table, the word combination is used in the strict technical sense. Each and
    // /// every combination of two cards that make a pair, of two or more cards that make 15, or of
    // /// three or more cards that make a run, count separately.
    // ///
    // /// Example: A hand (including the starter) comprised of 8, 7, 7, 6, 2 scores 8 points for four
    // /// combinations that total 15: the 8 with one 7, and the 8 with the other 7; the 6, 2 with each
    // /// of the two 7s. The same hand also scores 2 for a pair, and 6 for two runs of three (8, 7, 6
    // /// using each of the two 7s). The total score is 16. An experienced player computes the hand
    // /// thus: "Fifteen 2, fifteen 4, fifteen 6, fifteen 8, and 8 for double run is 16."
    // ///
    // /// Note that the ace is always low and cannot form a sequence with a king. Further, a flush
    // /// cannot happen during the play of the cards; it occurs only when the hands and the crib are
    // /// counted.
    // ///
    // /// Certain basic formulations should be learned to facilitate counting. For pairs and runs
    // /// alone:
    // ///
    // /// A. A triplet counts 6. A. Four of a kind counts 12. A. A run of three, with one card
    // /// duplicated (double run) counts 8. A. A run of four, with one card duplicated, counts 10. A.
    // /// A run of three, with one card triplicated (triple run), counts 15. A. A run of three, with
    // /// two different cards duplicated, counts 16.
    // // mod combinations {
    // //     use super::*;
    // //     use crate::{card, hand};
    // //     use std::str::FromStr;

    // //     #[test]
    // //     fn should_score_rules_example_eights_sevens_sixes() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("8H7C7D6S"), card!("2H"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(16)
    // //         );
    // //     }

    // //     #[test]
    // //     fn should_score_rules_example_runs() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("JHQCKDAS"), card!("2D"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(3)
    // //         );
    // //     }

    // //     #[test]
    // //     fn should_score_rules_example_flush() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("THQHKHAH"), card!("2H"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(5)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("THQHKHAH"), card!("2S"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(4)
    // //         );
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("THQHKHAS"), card!("2H"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(0)
    // //         );
    // //     }
    // // }

    // /// ### A PERFECT 29!
    // ///
    // /// The highest possible score for combinations in a single Cribbage deal is 29, and it may
    // /// occur only once in a Cribbage fan's lifetime -in fact, experts say that a 29 is probably as
    // /// rare as a hole-in-one in golf. To make this amazing score, a player must have a five as the
    // /// starter (upcard) and the other three fives plus the jack of the same suit as the starter -
    // /// His Nobs: 1 point - in his hand. The double pair royal (four 5s) peg another 12 points; the
    // /// various fives used to hit 15 can be done four ways for 8 points; and the jack plus a 5 to
    // /// hit 15 can also be done four ways for 8 points. Total = 29 points.
    // // mod a_perfect_29 {
    // //     use super::*;
    // //     use crate::{card, hand};
    // //     use std::str::FromStr;

    // //     #[test]
    // //     fn should_score_rules_example_perfect_29() {
    // //         assert_eq!(
    // //             HandScorer::new(&hand!("5H5C5DJS"), card!("5S"))
    // //                 .score()
    // //                 .points(),
    // //             Points::from(29)
    // //         );
    // //     }
    // // }

    // /// ## Miscellaneous
    // ///
    // /// The following list includes many of the hands that may give the beginner some difficulty in
    // /// counting. Note that no hand can make a count of 19, 25, 26, or 27. (In the chart below J
    // /// stands for His Nobs, the jack of the same suit as the starter.
    // ///
    // /// ### Muggins (optional) - not implemented.
    // ///
    // /// Each player must count his hand (and crib) aloud and announce the total. If he overlooks any
    // /// score, the opponent may say "Muggins" and then score the overlooked points for himself. For
    // /// experienced players, the Muggins rule is always in effect and adds even more suspense to the
    // /// game.
    // // mod miscellaneous {}

    // /// ## Game
    // ///
    // /// Game may be fixed at either 121 points or 61 points. The play ends the moment either player
    // /// reaches the agreed total, whether by pegging or counting one's hand. If the non-dealer "goes
    // /// out" by the count of his hand, the game immediately ends and the dealer may not score either
    // /// his hand or the crib.
    // ///
    // /// If a player wins the game before the loser has passed the halfway mark (did not reach 31 in
    // /// a game of 61, or 61 in a game of 121), the loser is "lurched," and the winner scores two
    // /// games instead of one. A popular variation of games played to 121, is a "skunk" (double game)
    // /// for the winner if the losing player fails to pass the three-quarter mark - 91 points or more -
    // /// and it is a "double skunk" (quadruple game) if the loser fails to pass the halfway mark (61
    // /// or more points).
    // // mod game {}

    // /// ## The Cribbage Board
    // ///
    // /// The Cribbage board (see illustration) has four rows of 30 holes each, divided into two pairs
    // /// of rows by a central panel. There are usually four (or two) additional holes near one end,
    // /// called "game holes." With the board come four pegs, usually in two contrasting colors. Note:
    // /// There are also continuous track Cribbage boards available which, as the name implies, have
    // /// one continuous line of 121 holes for each player.
    // ///
    // /// The board is placed to one side between the two players, and each player takes two pegs of
    // /// the same color. (The pegs are placed in the game holes until the game begins.) Each time a
    // /// player scores, he advances a peg along a row on his side of the board, counting one hole per
    // /// point. Two pegs are used, and the rearmost peg jumps over the first peg to show the first
    // /// increment in score. After another increase in score, the peg behind jumps over the peg in
    // /// front to the appropriate hole to show the player's new score, and so on (see diagram next
    // /// page). The custom is to "go down" (away from the game holes) on the outer rows and "come up"
    // /// on the inner rows. A game of 61 is "once around" and a game of 121 is "twice around." As
    // /// noted previously, continuous line Cribbage boards are available.
    // ///
    // /// If a Cribbage board is not available, each player may use a piece of paper or cardboard,
    // /// marked thus:
    // ///
    // ///   - Units 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    // ///   - Tens 10, 20, 30, 40, 50, 60
    // ///
    // /// Two small markers, such as small coins or buttons, can substitute for pegs for counting in
    // /// each row.
    // // mod the_cribbage_board {}

    // /// ## Strategy
    // ///
    // /// ### The Crib.
    // ///
    // /// If the dealer is discarding for the crib, he should “salt” it with the best possible cards,
    // /// but at the same time retain good cards in his hand that can be used for high scoring.
    // /// Conversely, for the non-dealer, it is best to lay out cards that will be the least
    // /// advantageous for the dealer. Laying out a five would be the worst choice, for the dealer
    // /// could use it to make 15 with any one of the ten-cards (10, J, Q, K). Laying out a pair is
    // /// usually a poor choice too, and the same goes for sequential cards, such as putting both a
    // /// six and seven in the crib. The ace and king tend to be good cards to put in the crib because
    // /// it is harder to use them in a run.
    // ///
    // /// ### The Play
    // ///
    // /// As expected, the five makes for the worst lead in that there are so many ten-cards that the
    // /// opponent can use to make a 15. Leading from a pair is a good idea, for even if the opponent
    // /// makes a pair, the leader can play the other matching card from his hand and collect for a
    // /// pair royal. Leading an ace or deuce is not a good idea, for these cards should be saved
    // /// until later to help make a 15, a Go, or a 31. The safest lead is a four because this card
    // /// cannot be used to make a 15 at the opponent’s very next turn. Finally, when the opponent
    // /// leads a card that can either be paired or make 15, the latter choice is preferred.
    // ///
    // /// During the play, it is advisable not to try to make a count of 21, for the opponent can then
    // /// play one of the many 10-cards and make 31 to gain two points.
    // // mod the_strategy {}

    // /// ## Internal
    // // mod internal {
    // //     use super::*;
    // //     use crate::{hand, ScoreComposition};
    // //     use std::str::FromStr;

    // //     fn common_filters() -> insta::Settings {
    // //         let mut settings = insta::Settings::new();
    // //         settings.add_filter(r"[0-9a-f]{8}", "<playerid>");
    // //         settings.add_filter(r"(A|[2-9]|T|J|Q|K)(H|C|D|S)", "<card>");
    // //         settings.add_filter(r"<card>(, <card>)*", "[<cards>]");
    // //         settings.add_filter(r"\s*\d+ ->\s*\d+", "<score>");
    // //         settings
    // //     }

    // //     #[test]
    // //     fn should_output_user_readable_starting_game_in_logs() {
    // //         let game = GameBuilder::default().with_cuts("ASAC").into_starting();
    // //         common_filters().bind(|| {
    // //             insta::assert_snapshot!(game.to_string(), @r"
    // //                 Starting(
    // //                     cuts: [<cards>],
    // //                     deck: Deck([<cards>])
    // //                 )
    // //                 ")
    // //         });
    // //     }

    // //     #[test]
    // //     fn should_output_user_readable_discarding_game_in_logs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("AH2H3H4H5H6H", "AC2C3C4C5C6C")
    // //             .into_discarding();
    // //         common_filters().bind(|| {
    // //             insta::assert_snapshot!(game.to_string(), @r"
    // //                 Discarding(
    // //                     scoreboard: Scores(<score>,<score>),
    // //                     roles: Dealer(Player(1)), Pone(Player(2)),
    // //                     hands: Hand([<cards>]), Hand([<cards>])
    // //                     crib: Crib(),
    // //                     deck: Deck([<cards>])
    // //                 )
    // //                 ")
    // //         });
    // //     }

    // //     #[test]
    // //     fn should_output_user_readable_playing_game_in_logs() {
    // //         let mut composition = ScoreComposition::default();
    // //         composition.with_fifteen(hand!("KS5S").as_ref(), Points::from(2));

    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_score_composition(composition)
    // //             .with_hands("9S", "4S")
    // //             .with_cut("AS")
    // //             .with_current_plays(&[(0, "AH")])
    // //             .into_playing(1);
    // //         common_filters().bind(|| insta::assert_snapshot!(game.to_string(), @r"
    // //                                      Playing(
    // //                                          scoreboard: Scores(<score>,<score>) Latest(Player(1): [Fifteen: [[<cards>]] => 2]),
    // //                                          roles: Dealer(Player(1)), Pone(Player(2)),
    // //                                          hands: Hand([<cards>]), Hand([<cards>]),
    // //                                          play_state: Next(Player(2)), Legal(), Passes(0), Current((Player(1) -> [<cards>])), Previous(),
    // //                                          cut: [<cards>],
    // //                                          crib: Crib()
    // //                                      )
    // //                                      "));
    // //     }

    // //     #[test]
    // //     fn should_output_user_readable_pone_scoring_game_in_logs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("AS2S3S4S", "AC2C3C4C")
    // //             .with_cut("JH")
    // //             .with_crib("TSJSQSKS")
    // //             .into_scoring_pone();
    // //         common_filters().bind(|| {
    // //             insta::assert_snapshot!(game.to_string(), @r"
    // //                                      ScoringPone(
    // //                                          scoreboard: Scores(<score>,<score>),
    // //                                          roles: Dealer(Player(1)), Pone(Player(2)),
    // //                                          hands: Hand([<cards>]), Hand([<cards>]),
    // //                                          cut: [<cards>],
    // //                                          crib: Crib([<cards>])
    // //                                      )
    // //                                      ")
    // //         });
    // //     }

    // //     #[test]
    // //     fn should_output_user_readable_dealer_scoring_game_in_logs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("AS2S3S4S", "AC2C3C4C")
    // //             .with_cut("JH")
    // //             .with_crib("TSJSQSKS")
    // //             .into_scoring_dealer();
    // //         common_filters().bind(|| {
    // //             insta::assert_snapshot!(game.to_string(), @r"
    // //                                      ScoringDealer(
    // //                                          scoreboard: Scores(<score>,<score>),
    // //                                          roles: Dealer(Player(1)), Pone(Player(2)),
    // //                                          hands: Hand([<cards>]), Hand([<cards>]),
    // //                                          cut: [<cards>],
    // //                                          crib: Crib([<cards>])
    // //                                      )
    // //                                      ")
    // //         });
    // //     }

    // //     #[test]
    // //     fn should_output_user_readable_crib_scoring_game_in_logs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 0)
    // //             .with_hands("AS2S3S4S", "AC2C3C4C")
    // //             .with_cut("JH")
    // //             .with_crib("TSJSQSKS")
    // //             .into_scoring_crib();
    // //         common_filters().bind(|| {
    // //             insta::assert_snapshot!(game.to_string(), @r"
    // //                                      ScoringCrib(
    // //                                          scoreboard: Scores(<score>,<score>),
    // //                                          roles: Dealer(Player(1)), Pone(Player(2)),
    // //                                          hands: Hand([<cards>]), Hand([<cards>]),
    // //                                          cut: [<cards>],
    // //                                          crib: Crib([<cards>])
    // //                                      )
    // //                                      ")
    // //         });
    // //     }

    // //     #[test]
    // //     fn should_output_user_readable_finished_game_in_logs() {
    // //         let game = GameBuilder::default()
    // //             .with_peggings(0, 121)
    // //             .with_winner(1)
    // //             .with_hands("AS2S3S4S", "AC2C3C4C")
    // //             .with_cut("JH")
    // //             .with_crib("TSJSQSKS")
    // //             .into_finished();
    // //         common_filters().bind(|| {
    // //             insta::assert_snapshot!(game.to_string(), @r"
    // //                                      Finished(
    // //                                          winner: Player(2),
    // //                                          scoreboard: Scores(<score>,<score>),
    // //                                          roles: Dealer(Player(1)), Pone(Player(2)),
    // //                                          hands: Hand([<cards>]), Hand([<cards>]),
    // //                                          crib: Crib([<cards>]),
    // //                                          cut: [<cards>]
    // //                                      )
    // //                                      ")
    // //         });
    // //     }
    // // }
    // TODO:
    // mod delete_me {}
}
