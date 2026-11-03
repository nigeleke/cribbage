use tellus::{ActorContext, Effect, EventSourced, Incoming, PersistenceId, Untimed};

use crate::{
    game::GameId,
    lobby::{
        command::LobbyCommand as C, error::LobbyError, event::LobbyEvent as E, id::LobbyId,
        state::LobbyState as S,
    },
};

/// Identifies a lobby.
pub struct Lobby {
    id: LobbyId,
}

impl Lobby {
    /// Creates a lobby with the specified identifier.
    pub fn new(id: LobbyId) -> Self {
        Self { id }
    }

    pub(crate) fn persistence_id(&self) -> PersistenceId {
        PersistenceId::new("lobby", self.id.to_string()).expect("valid persistence id required")
    }

    fn try_apply(&self, state: S, event: E) -> Result<S, LobbyError> {
        match (state, event) {
            (S::Unpaired, E::HostEntered { host }) => Ok(S::WaitingForGuest { host }),

            (S::WaitingForGuest { host }, E::GuestJoined { guest }) if host != guest => {
                let game = GameId::new();
                Ok(S::Paired { host, guest, game })
            }

            (S::WaitingForGuest { host }, E::GuestJoined { guest }) if host == guest => {
                Ok(S::WaitingForGuest { host })
            }

            (state, event) => {
                tracing::error!(
                    state = ?state,
                    event = ?event,
                    "Unhandled event for lobby state"
                );

                Err(LobbyError::UnhandledEvent)
            }
        }
    }
}

impl EventSourced for Lobby {
    type Command = C;
    type Event = E;
    type Timers = Untimed;
    type State = S;
    type Error = LobbyError;
    type Snapshot = S;

    fn persistence_id(&self) -> PersistenceId {
        self.persistence_id()
    }

    fn init(&self) -> Result<Self::State, LobbyError> {
        Ok(S::Unpaired)
    }

    fn init_from_snapshot(&self, snapshot: Self::Snapshot) -> Result<Self::State, Self::Error> {
        Ok(snapshot)
    }

    #[rustfmt::skip]
    fn handle(
        &self,
        _context: &ActorContext<Self::Command>,
        incoming: Incoming<Self::Command>,
        state: &S,
    ) -> Result<Effect<Self>, LobbyError> {
        let effect = match incoming {
            Incoming::Message(message) => match (message, state) {
                (
                    C::Create { host },
                    S::Unpaired
                ) => {
                    Effect::persist(E::HostEntered { host })
                }

                (
                    C::Join { guest },
                    S::WaitingForGuest { .. }
                ) => {
                    Effect::persist(E::GuestJoined { guest })
                },

                (_, _) => Effect::none(),
            }
            _ => Effect::none(),
        };

        Ok(effect)
    }

    fn apply(&self, state: S, event: E) -> S {
        println!("\n***** APPLY:   {:?}", event);
        println!("***** state:   {:?}", state);

        self.try_apply(state, event)
            .expect("event should be applied successfully")
    }
}
