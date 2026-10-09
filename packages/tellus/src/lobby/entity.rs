use cribbage_domain::prelude::{Card, CutForDealOutcome, Deck, Game as DomainGame, Player};
use tellus::{ActorContext, Effect, EventSourced, Incoming, Nothing, PersistenceId};

use crate::game::DeckSource;
use crate::lobby::{
    Acks, DeckSource, LobbyCommand as C, LobbyError, LobbyEvent as E, LobbyId, LobbyState as S,
};

pub struct Lobby<D: DeckSource> {
    id: LobbyId,
    deck_source: D,
}

impl<D: DeckSource> Lobby<D> {
    pub fn new(id: LobbyId, deck_source: D) -> Self {
        Self { id, deck_source }
    }

    pub(crate) fn persistence_id(&self) -> PersistenceId {
        PersistenceId::new("lobby", "instance").expect("valid persistence id required")
    }

    #[rustfmt::skip]
    fn try_apply(&self, state: C, event: E) -> Result<S, LobbyError> {
        match (state, event) {
            (
                S::PendingCreation,
                E::GameCreated { host, deck }
            ) => {
                let cards = Vec::<Card>::try_from(&deck).map_err(|_| GameError::InvalidDeck)?;
                let deck = Deck::from_iter(cards);
                Ok(GS::PendingGuest { host, deck })
            }

            (
                GS::PendingGuest { host, deck },
                GE::GuestJoined { guest }
            ) => {
                let game = DomainGame::new(deck);
                let users = Users::new(host, guest);
                Ok(GS::Starting { game, users })
            }

            (
                GS::PendingStartingAcks { game, users, mut acks },
                GE::CutForDealAcked { user }
            ) => {
                acks.ack(user);
                if acks.all_acked() {
                    Ok(GS::Starting { game, users })
                } else {
                    Ok(GS::PendingStartingAcks { game, users, acks })
                }
            }

            (
                GS::Starting { game, users },
                GE::CutForDealMade { user }
            ) => {
                let player = users[user].ok_or(GameError::UnknownUser(user))?;
                let acks = Acks::from(&users);
                match game.cut_for_deal(player) {
                    Ok(CutForDealOutcome::Dealing(_, game))  => Ok(GS::PendingDealingAcks { game, users, acks }),
                    Ok(CutForDealOutcome::Starting(_, game)) if game.cuts().is_empty() => Ok(GS::Starting { game, users}),
                    Err(e)                                   => Err(GameError::DomainError(e)),
                }
            }

            (
                GS::PendingDealingAcks { game, users, mut acks },
                GE::CutForDealAcked { user },
            ) => {
                acks.ack(user);

                let state = if acks.all_acked() {
                    GS::Dealing { game, users }
                } else {
                    GS::PendingDealingAcks { game, users, acks }
                };

                Ok(state)
            }

            (state, event) => {
                println!("***** unhandled event: {event:?}");
                println!("*****           state: {state:?}");
                Err(GameError::InvalidDeck)
            }
        }
    }
}

impl<D: DeckSource> EventSourced for Game<D> {
    type Command = GM;
    type Event = GE;
    type State = GS;
    type Error = GameError;
    type Snapshot = Nothing;

    fn persistence_id(&self) -> PersistenceId {
        self.persistence_id()
    }

    fn init(&self) -> Result<Self::State, Self::Error> {
        Ok(GS::PendingCreation)
    }

    fn init_from_snapshot(&self, snapshot: Self::Snapshot) -> Result<Self::State, Self::Error> {
        todo!()
    }

    #[rustfmt::skip]
    fn handle(
        &self,
        context: &ActorContext<Self::Command>,
        incoming: Incoming<Self::Command>,
        state: &Self::State,
    ) -> Result<Effect<Self>, Self::Error> {
        if let Incoming::Message(message) = &incoming {
            println!("\n***** HANDLE: {:?}", message);
            println!("***** state:   {:?}", state);
        };
        let effect = match incoming {
            Incoming::Message(message) => match (message, state) {
                (
                    GM::Create { host, deck },
                    GS::PendingCreation
                ) => {
                    Effect::persist(GE::GameCreated { host, deck })
                }

                (
                    GM::Join { guest },
                    GS::PendingGuest { host, deck }
                ) => {
                    Effect::persist(GE::GuestJoined { guest })
                }

                (
                    GM::Ack { user },
                    GS::PendingStartingAcks { game, users, acks }
                ) => {
                    let _ = users[user].ok_or(GameError::UnknownUser(user))?;
                    acks.ack_effect(user, Effect::persist(GE::CutForDealAcked { user }))
                }

                (
                    GM::CutForDeal { user },
                    GS::Starting { game, users }
                ) => {
                    let player = users[user].ok_or(GameError::UnknownUser(user))?;
                    game.validate_cut_for_deal(player)?;
                    Effect::persist(GE::CutForDealMade { user })
                }

                (
                    GM::Ack { user },
                    GS::PendingDealingAcks { game, users, acks }
                ) => {
                    let _ = users[user].ok_or(GameError::UnknownUser(user))?;
                    acks.ack_effect(user, Effect::persist(GE::CutForDealAcked { user }))
                }

                (
                    GM::DealHands { deck },
                    GS::Dealing { game, users }
                ) => {
                    let hands = {
                        let deck = Vec::<_>::try_from(&deck).map_err(|_| GameError::InvalidDeck)?;
                        let mut deck = Deck::from_iter(deck.into_iter());
                        game.validate_deal(&deck)?;
                        let hands = deck.deal();
                        [
                            PersistedCards::from(hands[Player::Player0].as_ref()),
                            PersistedCards::from(hands[Player::Player1].as_ref()),
                        ]
                    };
                    Effect::persist(GE::HandsDealt { hands, deck })
                }

                (GM::Discard { user, discard }, GS::Discarding { game, users }) => todo!(),
                (GM::CutStarter, GS::Cutting { game, users }) => todo!(),
                (GM::PlayCard { user, card }, GS::Playing { game, users }) => todo!(),
                (GM::Go { user }, GS::Playing { game, users }) => todo!(),
                (GM::ScorePone, GS::ScoringPone { game, users, acks }) => todo!(),
                (GM::Ack { user }, GS::ScoringPone { game, users, acks }) => todo!(),
                (GM::ScoreDealer, GS::ScoringDealer { game, users, acks }) => todo!(),
                (GM::Ack { user }, GS::ScoringDealer { game, users, acks }) => todo!(),
                (GM::ScoreCrib, GS::ScoringCrib { game, users, acks }) => todo!(),
                (GM::Ack { user }, GS::ScoringCrib { game, users, acks }) => todo!(),
                (_, _) => Effect::none(),
            },
            _ => Effect::none(),
        };

        Ok(effect)
    }

    fn apply(&self, state: Self::State, event: Self::Event) -> Self::State {
        println!("\n***** APPLY:   {:?}", event);
        println!("***** state:   {:?}", state);

        let state = self
            .try_apply(state, event)
            .expect("event should be applied successfully");

        println!("***** outcome: {:?}", state);

        state
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
