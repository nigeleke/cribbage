use pretty_assertions::assert_eq;
use tellus::{
    ActorSystem, Cbor, Codec, EncodedEvent, EventSourced, EventStore, InMemoryStore, Persistence,
    PersistenceId, ReplyTo, SeqNo, Versioned,
};

pub struct EntityFixture<E>
where
    E: EventSourced,
{
    system: ActorSystem<E::Command>,
    persistence_id: PersistenceId,
    givens_count: usize,
    event_store: InMemoryStore,
    snapshot_store: InMemoryStore,
}

pub struct AskedEntityFixture<E, R>
where
    E: EventSourced,
{
    fixture: EntityFixture<E>,
    reply: R,
}

impl<E> EntityFixture<E>
where
    E: EventSourced + Send + Sync + 'static,
    E::State: Send + 'static,
    E::Command: Send + 'static,
    E::Event: Send + 'static,
    E::Snapshot: Send + 'static,
{
    pub fn new(entity: E, persistence_id: PersistenceId) -> Self {
        let event_store = InMemoryStore::default();
        let snapshot_store = InMemoryStore::default();

        let system = ActorSystem::event_sourced(
            entity,
            Persistence::new(event_store.clone()).with_snapshot_store(snapshot_store.clone()),
        );

        Self {
            system,
            persistence_id,
            givens_count: 0,
            event_store,
            snapshot_store,
        }
    }

    fn persistence_id(&self) -> &PersistenceId {
        &self.persistence_id
    }

    pub async fn given(mut self, events: &[E::Event]) -> Self {
        let seq_no = self.event_store.events(self.persistence_id()).len();
        let encoded = events
            .iter()
            .map(|event| EncodedEvent {
                manifest: E::Event::MANIFEST.to_owned(),
                schema_version: E::Event::VERSION,
                payload: Cbor.encode(event).expect("event should encode"),
            })
            .collect::<Vec<_>>();

        self.event_store
            .append(self.persistence_id(), SeqNo::new(seq_no as u64), encoded)
            .await
            .expect("events should append to event store");

        self.givens_count += events.len();

        self
    }

    pub async fn given_event(self, event: &E::Event) -> Self {
        self.given(std::slice::from_ref(event)).await
    }

    pub async fn when_tell(self, command: E::Command) -> Self {
        self.system.root().tell(command);
        tokio::task::yield_now().await;
        self
    }

    pub async fn when_ask<R, F>(self, command: F) -> AskedEntityFixture<E, R>
    where
        R: Send + 'static,
        F: FnOnce(ReplyTo<R>) -> E::Command,
    {
        let reply = self
            .system
            .root()
            .ask(std::time::Duration::from_secs(1), command)
            .await
            .expect("ask must not timeout");

        AskedEntityFixture {
            fixture: self,
            reply,
        }
    }

    pub fn then_events(self, expected: &[E::Event]) -> Self
    where
        E::Event: std::fmt::Debug + PartialEq,
    {
        let stored = self.event_store.events(self.persistence_id());
        let actual = (&stored[self.givens_count..])
            .iter()
            .map(|stored| {
                E::Event::decode(&Cbor, stored.event.schema_version, &stored.event.payload)
                    .expect("stored event should decode")
            })
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);

        self
    }

    pub fn then_snapshot(self, expected: Option<&E::Snapshot>) -> Self
    where
        E::Snapshot: PartialEq + std::fmt::Debug,
    {
        let actual = self
            .snapshot_store
            .snapshot(self.persistence_id())
            .map(|stored| {
                E::Snapshot::decode(
                    &Cbor,
                    stored.snapshot.schema_version,
                    &stored.snapshot.payload,
                )
                .expect("stored snapshot should decode")
            });

        assert_eq!(actual.as_ref(), expected);

        self
    }
}

impl<E, R> AskedEntityFixture<E, R>
where
    E: EventSourced,
    R: PartialEq + std::fmt::Debug,
{
    pub fn then_reply(self, expected: R) -> EntityFixture<E>
    where
        R: PartialEq + std::fmt::Debug,
    {
        assert_eq!(self.reply, expected);
        self.fixture
    }
}
