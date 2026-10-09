use core::panic;

use cribbage_domain::prelude::{
    CutForDealOutcome, DealOutcome, Deck, Discard, DiscardOutcome, Game as DomainGame,
    GameError as DomainGameError, Player,
};

use cribbage_tellus::prelude::{
    DeckSource, Game, GameCommand, GameEvent, GameId, GameState, PersistedCard, PersistedCards,
    UserId, Users,
};
use tellus::EventSourced;

use crate::test_support::{EntityFixture, ShuffledDeckSource};

pub struct GameScenario {
    id: GameId,
    state: GameState,
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
            _ => panic!("invalid state for action"),
        };

        GameScenario { id, state, events }
    }};
}

impl GameScenario {
    pub fn users(&self) -> Users {
        match self.state {
            GameState::PendingCreation => panic!("no users"),
            GameState::Starting { users, .. }
            | GameState::Dealing { users, .. }
            | GameState::Discarding { users, .. }
            | GameState::Cutting { users, .. }
            | GameState::Playing { users, .. }
            | GameState::ScoringPone { users, .. }
            | GameState::ScoringDealer { users, .. }
            | GameState::ScoringCrib { users, .. }
            | GameState::Finished { users, .. } => users,
        }
    }

    pub fn progress_to_starting(self) -> Self {
        self.create_game()
    }

    pub fn progress_to_dealing(self) -> Self {
        let starting = self.progress_to_starting();

        let host = starting.users().user(Player::Player0);
        let guest = starting.users().user(Player::Player1);

        starting.cut_for_deal(host).cut_for_deal(guest)
    }

    pub async fn to_fixture(&self) -> EntityFixture<Game<ShuffledDeckSource>> {
        let game = Game::new(self.id, ShuffledDeckSource::default());
        let id = game.persistence_id();
        EntityFixture::new(game, id).given(&self.events).await
    }

    fn create_game(self) -> Self {
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

    fn cut_for_deal(self, user: UserId) -> Self {
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

    fn deal(self, deck: Deck) -> Self {
        transition!(
            self,
            GameState::Dealing { game, users },
            {
                let persisted_deck = PersistedCards::from(deck.as_ref());
                game.deal(deck).map(|outcome| (outcome, persisted_deck))
            },
            (DealOutcome::Discarding(game), persisted_deck) => (
                    GameState::Discarding { game, users },
                    [GameEvent::HandsDealt { deck: persisted_deck }]
            )
        )
    }

    fn discard(self, user: UserId, discard: Discard) -> Self {
        transition!(
            self,
            GameState::Discarding { game, users },
            {
                let player = users.player(user).expect("valid user required");
                game.discard(player, discard)
            },
            DiscardOutcome::Discarding(game) => (GameState::Discarding { game, users }, vec![]),
            DiscardOutcome::Cutting(game) => (GameState::Cutting { game, users }, vec![]),
        )
    }

    fn cut_starter(mut self, starter: PersistedCard) -> Self {
        self.events.push(GameEvent::StarterCut { starter });
        self
    }

    fn play_card(mut self, user: UserId, card: PersistedCard) -> Self {
        self.events.push(GameEvent::CardPlayed { user, card });
        self
    }

    fn go(mut self, user: UserId) -> Self {
        self.events.push(GameEvent::GoMade { user });
        self
    }

    fn score_pone(mut self) -> Self {
        self.events.push(GameEvent::PoneScoreMade);
        self
    }

    fn score_dealer(mut self) -> Self {
        self.events.push(GameEvent::DealerScoreMade);
        self
    }

    fn score_crib(mut self) -> Self {
        self.events.push(GameEvent::CribScoreMade);
        self
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
