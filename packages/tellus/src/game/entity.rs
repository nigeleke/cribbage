use cribbage_domain::prelude::{
    CutForDealOutcome, CutStarterOutcome, DealOutcome, DiscardOutcome, Game as DomainGame,
    GoOutcome, PlayOutcome, ScoreCribOutcome, ScoreDealerOutcome, ScorePoneOutcome,
};
use tellus::{ActorContext, Effect, EventSourced, Incoming, PersistenceId};

use crate::game::{
    command::GameCommand as C, error::GameError, event::GameEvent as E, id::GameId,
    state::GameState as S,
};

/// Identifies a cribbage game.
pub struct Game {
    id: GameId,
}

impl Game {
    /// Creates a game with the specified identifier.
    pub fn new(id: GameId) -> Self {
        Self { id }
    }

    pub(crate) fn persistence_id(&self) -> PersistenceId {
        PersistenceId::new("game", self.id.to_string())
            .expect("game id produces a valid persistence id")
    }

    fn try_apply(&self, state: S, event: E) -> Result<S, GameError> {
        match (state, event) {
            (S::PendingCreation, E::GameCreated { users, deck }) => {
                let game = DomainGame::new(deck);
                Ok(S::Starting { game, users })
            }

            (S::Starting { game, users }, E::CutForDealMade { user }) => {
                let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                match game.cut_for_deal(player) {
                    Ok(CutForDealOutcome::Dealing(_, game)) => Ok(S::Dealing { game, users }),
                    Ok(CutForDealOutcome::Starting(_, game)) => Ok(S::Starting { game, users }),
                    Err(e) => Err(GameError::DomainError(e)),
                }
            }

            (S::Dealing { game, users }, E::HandsDealt { deck, .. }) => match game.deal(deck) {
                Ok(DealOutcome::Discarding(game)) => Ok(S::Discarding { game, users }),
                Err(e) => Err(GameError::DomainError(e)),
            },

            (S::Discarding { game, users }, E::DiscardMade { user, discard }) => {
                let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                match game.discard(player, discard) {
                    Ok(DiscardOutcome::Discarding(game)) => Ok(S::Discarding { game, users }),
                    Ok(DiscardOutcome::Cutting(game)) => Ok(S::Cutting { game, users }),
                    Err(e) => Err(GameError::DomainError(e)),
                }
            }

            (S::Cutting { game, users }, E::StarterCut) => match game.cut_starter() {
                Ok(CutStarterOutcome::Playing(game)) => Ok(S::Playing { game, users }),
                Ok(CutStarterOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                Err(e) => Err(GameError::DomainError(e)),
            },

            (S::Playing { game, users }, E::CardPlayed { user, card }) => {
                let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                match game.play(player, card) {
                    Ok(PlayOutcome::Playing(game)) => Ok(S::Playing { game, users }),
                    Ok(PlayOutcome::Scoring(game)) => Ok(S::ScoringPone { game, users }),
                    Ok(PlayOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                    Err(e) => Err(GameError::DomainError(e)),
                }
            }

            (S::Playing { game, users }, E::GoMade { user }) => {
                let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                match game.go(player) {
                    Ok(GoOutcome::Playing(game)) => Ok(S::Playing { game, users }),
                    Ok(GoOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                    Err(e) => Err(GameError::DomainError(e)),
                }
            }

            (S::ScoringPone { game, users }, E::PoneScoreMade) => match game.score_pone() {
                Ok(ScorePoneOutcome::Scoring(game)) => Ok(S::ScoringDealer { game, users }),
                Ok(ScorePoneOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                Err(e) => Err(GameError::DomainError(e)),
            },

            (S::ScoringDealer { game, users }, E::DealerScoreMade) => match game.score_dealer() {
                Ok(ScoreDealerOutcome::Scoring(game)) => Ok(S::ScoringCrib { game, users }),
                Ok(ScoreDealerOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                Err(e) => Err(GameError::DomainError(e)),
            },

            (S::ScoringCrib { game, users }, E::CribScoreMade) => match game.score_crib() {
                Ok(ScoreCribOutcome::Dealing(game)) => Ok(S::Dealing { game, users }),
                Ok(ScoreCribOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                Err(e) => Err(GameError::DomainError(e)),
            },

            (state, event) => {
                tracing::error!(
                    state = ?state,
                    event = ?event,
                    "Unhandled event for game state"
                );

                Err(GameError::UnhandledEvent)
            }
        }
    }
}

impl EventSourced for Game {
    type Command = C;
    type Event = E;
    type State = S;
    type Error = GameError;
    type Snapshot = S;

    fn persistence_id(&self) -> PersistenceId {
        self.persistence_id()
    }

    fn init(&self) -> Result<Self::State, Self::Error> {
        Ok(S::PendingCreation)
    }

    fn init_from_snapshot(&self, snapshot: Self::Snapshot) -> Result<Self::State, Self::Error> {
        Ok(snapshot)
    }

    fn handle(
        &self,
        _context: &ActorContext<Self::Command>,
        incoming: Incoming<Self::Command>,
        state: &Self::State,
    ) -> Result<Effect<Self>, Self::Error> {
        let effect = match incoming {
            Incoming::Message(message) => match (message, state) {
                (C::CreateGame { users, deck }, S::PendingCreation) => {
                    Effect::persist(E::GameCreated { users, deck })
                }

                (C::CutForDeal { user }, S::Starting { game, users }) => {
                    let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                    game.validate_cut_for_deal(player)?;
                    Effect::persist(E::CutForDealMade { user })
                }

                (C::DealHands { deck }, S::Dealing { game, .. }) => {
                    game.validate_deal(&deck)?;
                    Effect::persist(E::HandsDealt { deck })
                }

                (C::Discard { user, discard }, S::Discarding { game, users }) => {
                    let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                    game.validate_discard(player, discard)?;
                    Effect::persist(E::DiscardMade { user, discard })
                }

                (C::CutStarter, S::Cutting { game, .. }) => {
                    game.validate_cut_starter()?;
                    Effect::persist(E::StarterCut)
                }

                (C::PlayCard { user, card }, S::Playing { game, users }) => {
                    let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                    game.validate_play(player, card)?;
                    Effect::persist(E::CardPlayed { user, card })
                }

                (C::Go { user }, S::Playing { game, users }) => {
                    let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                    game.validate_go(player)?;
                    Effect::persist(E::GoMade { user })
                }

                (C::ScorePone, S::ScoringPone { .. }) => Effect::persist(E::PoneScoreMade),

                (C::ScoreDealer, S::ScoringDealer { .. }) => Effect::persist(E::DealerScoreMade),

                (C::ScoreCrib, S::ScoringCrib { .. }) => Effect::persist(E::CribScoreMade),

                (_, _) => Effect::none(),
            },
            _ => Effect::none(),
        };

        Ok(effect)
    }

    fn apply(&self, state: Self::State, event: Self::Event) -> Self::State {
        self.try_apply(state, event)
            .expect("event should be applied successfully")
    }
}
