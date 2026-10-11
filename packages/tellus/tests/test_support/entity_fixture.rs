use tellus::{Effect, EventSourced, Incoming, testing::TestContext};

pub struct EntityFixture<E: EventSourced>
where
    E::Command: Send + 'static,
    E::Event: Clone,
{
    context: TestContext<E::Command>,
    entity: E,
    state: E::State,
    when_result: Option<Result<Effect<E>, E::Error>>,
}

impl<E: EventSourced> EntityFixture<E>
where
    E::Command: Send + 'static,
    E::Event: Clone,
{
    pub fn new(entity: E) -> Self {
        let context = TestContext::new();
        let state = entity.init().expect("entity must be initialised");

        Self {
            context,
            entity,
            state,
            when_result: None,
        }
    }

    pub fn given(self, events: impl Into<Vec<E::Event>>) -> Self {
        let Self {
            context,
            state,
            entity,
            when_result,
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
            when_result,
        }
    }

    pub fn given_event(self, event: E::Event) -> Self {
        self.given(&[event])
    }

    pub fn when(mut self, command: E::Command) -> Self {
        let result = self.entity.handle(
            self.context.context(),
            Incoming::Message(command),
            &self.state,
        );

        let events = result
            .as_ref()
            .ok()
            .map(|effect| effect.events().into())
            .unwrap_or(Vec::new());

        self.when_result = Some(result);

        self.given(events)
    }

    pub fn then(self, assert: impl Fn(&E::State) -> ()) -> Self
    where
        E::State: std::fmt::Debug + PartialEq,
    {
        match self.when_result {
            Some(Ok(_)) => assert(&self.state),
            Some(Err(e)) => panic!("when condition failed; expected ok, was error: {e}"),
            None => panic!("when not executed"),
        }

        self
    }

    pub fn then_error(self, assert: impl Fn(&E::Error) -> ()) -> Self
    where
        E::Error: std::fmt::Debug + PartialEq,
    {
        match &self.when_result {
            Some(Ok(_)) => panic!("when condition succeeded; expected error"),
            Some(Err(actual)) => assert(actual),
            None => panic!("when not executed"),
        }

        self
    }
}
