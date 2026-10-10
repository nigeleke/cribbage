use pretty_assertions::assert_eq;
use tellus::{EventSourced, Incoming, testing::TestContext};

pub struct EntityFixture<E: EventSourced> {
    context: TestContext<E::Command>,
    entity: E,
    state: E::State,
}

impl<E: EventSourced> EntityFixture<E> {
    pub fn new(entity: E) -> Self
    where
        E::Command: Send + 'static,
    {
        let context = TestContext::new();
        let state = entity.init().expect("entity must be initialised");

        Self {
            context,
            entity,
            state,
        }
    }

    pub fn given(self, events: impl Into<Vec<E::Event>>) -> Self {
        let Self {
            context,
            state,
            entity,
        } = self;

        let events = events.into();

        let (new_state, updated_entity) =
            events
                .into_iter()
                .fold((state, entity), |(state, entity), event| {
                    let state = entity.apply(state, event);
                    (state, entity)
                });

        Self {
            context,
            entity: updated_entity,
            state: new_state,
        }
    }

    pub fn given_event(self, event: E::Event) -> Self {
        self.given([event])
    }

    pub fn when(self, command: E::Command) -> Self {
        self.entity
            .handle(
                self.context.context(),
                Incoming::Message(command),
                &self.state,
            )
            .expect("command must be sucessful");

        self
    }

    pub fn then(self, expected: &E::State) -> Self
    where
        E::State: std::fmt::Debug + PartialEq,
    {
        assert_eq!(&self.state, expected);
        self
    }
}
