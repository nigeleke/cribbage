use cribbage_domain::prelude::{
    Card, CutForDealOutcome, CutStarterOutcome, DealOutcome, Deck, Discard, DiscardOutcome,
    Game as DomainGame, PlayOutcome,
};
use tellus::{ActorContext, Effect, EventSourced, Incoming, Nothing, PersistenceId};

use crate::game::{
    DeckSource, GameCommand as C, GameError,
    GameEvent::{self as E, StarterCut},
    GameId, GameState as S,
};
use crate::persistence::PersistedCard;

pub struct Game<D: DeckSource> {
    id: GameId,
    _deck_source: D,
}

impl<D: DeckSource> Game<D> {
    pub fn new(id: GameId, _deck_source: D) -> Self {
        Self { id, _deck_source }
    }

    pub(crate) fn persistence_id(&self) -> PersistenceId {
        PersistenceId::new("game", self.id.to_string())
            .expect("game id produces a valid persistence id")
    }

    fn try_apply(&self, state: S, event: E) -> Result<S, GameError> {
        match (state, event) {
            (S::PendingCreation, E::GameCreated { users, deck }) => {
                let cards = Vec::<Card>::try_from(&deck).map_err(|_| GameError::InvalidDeck)?;
                let deck = Deck::from_iter(cards);
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

            (S::Dealing { game, users }, E::HandsDealt { deck, .. }) => {
                let cards = Vec::<Card>::try_from(&deck).map_err(|_| GameError::InvalidDeck)?;
                let deck = Deck::from_iter(cards);
                match game.deal(deck) {
                    Ok(DealOutcome::Discarding(game)) => Ok(S::Discarding { game, users }),
                    Err(e) => Err(GameError::DomainError(e)),
                }
            }

            (S::Discarding { game, users }, E::DiscardMade { user, discard }) => {
                let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                let discard1 =
                    Discard::try_from(&discard).map_err(|_| GameError::InvalidDiscard)?;

                match game.discard(player, discard1) {
                    Ok(DiscardOutcome::Discarding(game)) => Ok(S::Discarding { game, users }),
                    Ok(DiscardOutcome::Cutting(game)) => Ok(S::Cutting { game, users }),
                    Err(e) => Err(GameError::DomainError(e)),
                }
            }

            (S::Cutting { game, users }, StarterCut) => match game.cut_starter() {
                Ok(CutStarterOutcome::Playing(game)) => Ok(S::Playing { game, users }),
                Ok(CutStarterOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                Err(e) => Err(GameError::DomainError(e)),
            },

            (S::Playing { game, users }, E::CardPlayed { user, card }) => {
                let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                let card = Card::try_from(&card).map_err(|_| GameError::InvalidCard)?;
                match game.play(player, card) {
                    Ok(PlayOutcome::Playing(game)) => Ok(S::Playing { game, users }),
                    Ok(PlayOutcome::Scoring(game)) => Ok(S::ScoringPone { game, users }),
                    Ok(PlayOutcome::Finished(game)) => Ok(S::Finished { game, users }),
                    Err(e) => Err(GameError::DomainError(e)),
                }
            }

            (state, event) => {
                panic!(
                    r#"***** unhandled state/event in apply
                    event: {event:?}
                    state: {state:?}"#
                );
            }
        }
    }
}

impl<D: DeckSource> EventSourced for Game<D> {
    type Command = C;
    type Event = E;
    type State = S;
    type Error = GameError;
    type Snapshot = Nothing;

    fn persistence_id(&self) -> PersistenceId {
        self.persistence_id()
    }

    fn init(&self) -> Result<Self::State, Self::Error> {
        Ok(S::PendingCreation)
    }

    fn init_from_snapshot(&self, _snapshot: Self::Snapshot) -> Result<Self::State, Self::Error> {
        todo!()
    }

    fn handle(
        &self,
        _context: &ActorContext<Self::Command>,
        incoming: Incoming<Self::Command>,
        state: &Self::State,
    ) -> Result<Effect<Self>, Self::Error> {
        if let Incoming::Message(message) = &incoming {
            println!("\n***** HANDLE: {:?}", message);
            println!("***** state:   {:?}", state);
        };
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
                    let deck1 = Vec::<_>::try_from(&deck).map_err(|_| GameError::InvalidDeck)?;
                    let deck1 = Deck::from_iter(deck1);
                    game.validate_deal(&deck1)?;

                    Effect::persist(E::HandsDealt { deck })
                }

                (C::Discard { user, discard }, S::Discarding { game, users }) => {
                    let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                    let discard1 =
                        Discard::try_from(&discard).map_err(|_| GameError::InvalidDiscard)?;
                    game.validate_discard(player, discard1)?;

                    Effect::persist(E::DiscardMade { user, discard })
                }

                (C::CutStarter, S::Cutting { game, .. }) => {
                    game.validate_cut_starter()?;
                    Effect::persist(E::StarterCut)
                }

                (C::PlayCard { user, card }, S::Playing { game, users }) => {
                    let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
                    let card0 = Card::try_from(&card).map_err(|_| GameError::InvalidCard)?;
                    game.validate_play(player, card0)?;
                    Effect::persist(E::CardPlayed { user, card })
                }

                (C::Go { user }, S::Playing { game, users }) => todo!(),
                (C::ScorePone, S::ScoringPone { game, users }) => todo!(),
                (C::ScoreDealer, S::ScoringDealer { game, users }) => todo!(),
                (C::ScoreCrib, S::ScoringCrib { game, users }) => todo!(),
                (_, _) => Effect::none(),
            },
            _ => Effect::none(),
        };

        Ok(effect)
    }

    fn apply(&self, state: Self::State, event: Self::Event) -> Self::State {
        println!("\n***** APPLY:   {:?}", event);
        println!("***** state:   {:?}", state);

        self.try_apply(state, event)
            .expect("event should be applied successfully")
    }

    //
    //     fn receive(
    //         &self,
    //         _: &ActorContext<Self::Message>,
    //         incoming: Incoming<Self::Message>,
    //         state: Self::State,
    //     ) -> Result<Control<Self::State>, Self::Error> {
    //         let Incoming::Message(message) = incoming else {
    //             return Ok(Control::Continue(state));
    //         };
    //
    //         let state = match (state, message) {
    //             (GameState::AwaitingCreation, GameMessage::Create { host, deck }) => {
    //                 GameState::WaitingForPlayer { host, deck }
    //             }
    //
    //             (GameState::WaitingForPlayer { host, deck }, GameMessage::Join { guest }) => {
    //                 GameState::Started {
    //                     game: DomainGame::new(deck),
    //                     users: Users::new(host, guest),
    //                 }
    //             }
    //
    //             (GameState::Started { game, users }, GameMessage::CutForDeal { user, acks }) => {
    //                 let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
    //                 match game.cut_for_deal(player)? {
    //                     CutForDealOutcome::Starting(game) => GameState::Started { game, users },
    //                     CutForDealOutcome::Dealing(game) => GameState::Dealing { game, users },
    //                 }
    //             }
    //
    //             (GameState::Started { game, users }, GameMessage::Ack { user }) => {
    //                 let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
    //                 match game.cut_for_deal(player)? {
    //                     CutForDealOutcome::Starting(game) => GameState::Started { game, users },
    //                     CutForDealOutcome::Dealing(game) => GameState::Dealing { game, users },
    //                 }
    //             }
    //
    //             (GameState::Dealing { game, users }, GameMessage::Deal { deck }) => {
    //                 match game.deal(deck)? {
    //                     DealOutcome::Discarding(game) => GameState::Discarding { game, users },
    //                 }
    //             }
    //
    //             (GameState::Discarding { game, users }, GameMessage::Discard { user, discard }) => {
    //                 let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
    //                 match game.discard(player, discard)? {
    //                     DiscardOutcome::Discarding(game) => GameState::Discarding { game, users },
    //                     DiscardOutcome::Cutting(game) => GameState::Cutting { game, users },
    //                 }
    //             }
    //
    //             (GameState::Cutting { game, users }, GameMessage::CutStarter { acks }) => {
    //                 match game.cut_starter()? {
    //                     CutStarterOutcome::Playing(game) => GameState::Playing { game, users },
    //                     CutStarterOutcome::Finished(game) => GameState::Finished { game, users },
    //                 }
    //             }
    //
    //             (GameState::Playing { game, users }, GameMessage::PlayCard { user, card }) => {
    //                 let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
    //                 match game.play(player, card)? {
    //                     PlayOutcome::Playing(game) => GameState::Playing { game, users },
    //                     PlayOutcome::Scoring(game) => GameState::ScoringPone { game, users },
    //                     PlayOutcome::Finished(game) => GameState::Finished { game, users },
    //                 }
    //             }
    //
    //             (GameState::Playing { game, users }, GameMessage::Go { user }) => {
    //                 let player = users.player(user).ok_or(GameError::UnknownUser(user))?;
    //                 match game.go(player)? {
    //                     GoOutcome::Playing(game) => GameState::Playing { game, users },
    //                     GoOutcome::Scoring(game) => GameState::ScoringPone { game, users },
    //                     GoOutcome::Finished(game) => GameState::Finished { game, users },
    //                 }
    //             }
    //
    //             (GameState::ScoringPone { game, users }, GameMessage::ScorePone { acks }) => {
    //                 match game.score_pone()? {
    //                     ScorePoneOutcome::Scoring(game) => GameState::ScoringDealer { game, users },
    //                     ScorePoneOutcome::Finished(game) => GameState::Finished { game, users },
    //                 }
    //             }
    //
    //             (GameState::ScoringDealer { game, users }, GameMessage::ScoreDealer { acks }) => {
    //                 match game.score_dealer()? {
    //                     ScoreDealerOutcome::Scoring(game) => GameState::ScoringCrib { game, users },
    //                     ScoreDealerOutcome::Finished(game) => GameState::Finished { game, users },
    //                 }
    //             }
    //
    //             (GameState::ScoringCrib { game, users }, GameMessage::ScoreCrib { acks }) => {
    //                 match game.score_crib()? {
    //                     ScoreCribOutcome::Dealing(game) => GameState::Dealing { game, users },
    //                     ScoreCribOutcome::Finished(game) => GameState::Finished { game, users },
    //                 }
    //             }
    //
    //             (state, _) => state,
    //         };
    //
    //         Ok(Control::Continue(state))
    //     }
    //
}
