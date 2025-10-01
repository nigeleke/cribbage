use eventsourced::{Command, CommandEffect, EventSourced};

use crate::TestFrameworkResult;

/// A testing utility for simulating command handling and event sourcing in a CQRS/event-sourced system.
///
/// `TestFramework` provides a way to construct an entity from historical events (`given`)
/// and apply new commands (`when`), capturing the resulting `CommandEffect` for assertions
/// in tests.
///
/// # Type Parameters
/// - `E`: The entity type under test, which must implement the [`EventSourced`] trait.
///
/// # Fields
/// - `id`: The identifier of the entity under test.
/// - `entity`: The current state of the entity, updated via historical events or command effects.
pub struct TestFramework<E>
where
    E: EventSourced,
{
    id: E::Id,
    entity: E,
}

impl<E> TestFramework<E>
where
    E: EventSourced,
{
    /// Create new `TestFramework` for entity.
    ///
    /// # Parameters
    /// - `id` the entity id.
    /// - `entity` the entity itself.
    pub fn new(id: E::Id, entity: E) -> Self {
        Self { id, entity }
    }

    /// Assert conditions (using supplied function) on an entity.
    ///
    /// # Parameters
    /// - `f` function predicating on current entity. The function must
    ///   panic if predicates fail.
    ///
    /// # Returns
    /// Self to enable chaining.
    pub fn assert_entity(self, f: impl Fn(&E)) -> Self {
        f(&self.entity);
        self
    }

    /// The current entity.
    /// If `given` has been called this will have those events applied.
    pub fn entity(&self) -> &E {
        &self.entity
    }

    /// Applies a sequence of historical events to the test framework's entity, simulating
    /// the past evolution of its state.
    ///
    /// # Parameters
    /// - `events`: A vector of events to apply in order to the entity.
    ///
    /// # Returns
    /// A new `TestFramework` instance with the entity in the state resulting
    /// from applying all the given events.
    pub fn given(self, events: Vec<E::Event>) -> Self
    where
        E::Event: Clone,
    {
        events.into_iter().fold(self, |mut me, event| {
            me.entity = me.entity.handle_event(event.clone());
            me
        })
    }

    /// Applies a command to the current state of the entity and captures the resulting effect.
    ///
    /// The resulting state and effect are wrapped in a `TestFrameworkResult`, which
    /// can then be inspected using test assertions (e.g., checking emitted events or replies).
    ///
    /// # Type Parameters
    /// - `R`: The type of reply returned by the command.
    /// - `ER`: The type of error returned if the command is rejected.
    ///
    /// # Parameters
    /// - `command`: The command to be applied to the entity.
    ///
    /// # Returns
    /// A [`TestFrameworkResult`] containing the entity and the resulting effect from the command.
    ///
    /// # Constraints
    /// - `E::Event`: Must implement [`PartialEq`] for event comparison in tests.
    /// - `R`: Must implement [`PartialEq`] and [`Debug`] for reply comparison in tests.
    /// - `ER`: Must implement [`PartialEq`] and [`Debug`] for error comparison in tests.
    pub fn when<R, ER>(
        mut self,
        command: impl Command<E, Reply = R, Error = ER>,
    ) -> TestFrameworkResult<E, R, ER>
    where
        E::Event: Clone + PartialEq,
        R: PartialEq + std::fmt::Debug,
        ER: PartialEq + std::fmt::Debug,
    {
        let effect = command.handle_command(&self.id, &self.entity);

        if let CommandEffect::EmitAndReply(event, _) = &effect {
            self.entity = self.entity.handle_event(event.clone());
        }

        TestFrameworkResult::new(self.entity, effect)
    }
}
