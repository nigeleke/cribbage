use eventsourced::{CommandEffect, EventSourced};

/// The result of executing a command in the [`TestFramework`], capturing the effects
/// such as emitted events, replies, or errors.
///
/// This struct is designed to hold the outcome of applying a command to an event-sourced
/// entity during testing. It is used to inspect and assert the behavior of aggregates
/// under test by observing emitted events, command replies, or rejections.
///
/// # Type Parameters
/// - `E`: The entity type under test, implementing the [`EventSourced`] trait.
/// - `R`: The reply type produced by successful command execution.
/// - `ER`: The error type produced if the command is rejected.
///
/// # Fields
/// - `event`: The event emitted by the entity (if any).
/// - `reply`: The reply returned by the command handler (if successful).
/// - `error`: The error returned by the command handler (if rejected).
pub struct TestFrameworkResult<E, R, ER>
where
    E: EventSourced,
{
    entity: E,
    event: Option<E::Event>,
    reply: Option<R>,
    error: Option<ER>,
}

impl<E, R, ER> TestFrameworkResult<E, R, ER>
where
    E: EventSourced,
    E::Event: PartialEq,
    R: PartialEq + std::fmt::Debug,
    ER: PartialEq + std::fmt::Debug,
{
    /// Constructs a new [`TestFrameworkResult`] from a [`CommandEffect`] and the given entity.
    ///
    /// # Parameters
    /// - `entity`: The aggregate instance on which the command was executed. This is required
    ///   in order to compute the reply for [`CommandEffect::EmitAndReply`], which uses a closure
    ///   that takes a reference to the entity.
    /// - `effect`: The effect produced by executing the command. This may include:
    ///     - an event and a function producing a reply (`EmitAndReply`)
    ///     - a direct reply (`Reply`)
    ///     - a rejection error (`Reject`)
    ///
    /// # Returns
    /// A [`TestFrameworkResult`] containing the appropriate `event`, `reply`, and/or `error`
    /// extracted from the effect.
    ///
    /// # Note
    /// This method evaluates the reply function immediately in the case of `EmitAndReply`.
    pub fn new(entity: E, effect: CommandEffect<E, R, ER>) -> Self {
        match effect {
            CommandEffect::EmitAndReply(event, f) => {
                let event = Some(event);
                let reply = Some(f(&entity));
                Self {
                    entity,
                    event,
                    reply,
                    error: None,
                }
            }
            CommandEffect::Reply(reply) => {
                let reply = Some(reply);
                Self {
                    entity,
                    event: None,
                    reply,
                    error: None,
                }
            }
            CommandEffect::Reject(error) => {
                let error = Some(error);
                Self {
                    entity,
                    event: None,
                    reply: None,
                    error,
                }
            }
        }
    }

    /// Assert conditions (using supplied function) on an entity.
    ///
    /// # Parameters
    /// - `f` function predicating on the entity after events have been applied.
    /// The function must panic if predicates fail.
    ///
    /// # Returns
    /// Self to enable chaining.
    pub fn assert_entity(self, f: impl Fn(&E)) -> Self {
        f(&self.entity);
        self
    }

    /// Return the resultant entity, before the emitted events have been applied.
    pub fn entity(&self) -> &E {
        &self.entity
    }

    /// Asserts that the effect emitted the expected event.
    ///
    /// # Parameters
    /// - `expected_event`: The event you expect the command to have emitted.
    ///
    /// # Panics
    /// - Panics if no event was emitted (i.e. `self.event()` is `None`).
    /// - Panics if the emitted event does not equal the expected event.
    ///
    /// # Returns
    /// Returns `self`, allowing this method to be chained with other expectations.
    ///
    /// # Requirements
    /// The `E::Event` type must implement [`PartialEq`] and [`Debug`] for `assert_eq!`.
    pub fn expect_event(self, expected_event: E::Event) -> Self {
        let actual_event = self.event();
        assert_eq!(actual_event, &expected_event);
        self
    }

    /// Asserts (using supplied function) that the effect emitted
    /// the expected event.
    ///
    /// # Parameters
    /// - `f`: A function predicating the actual event. This function should
    /// panic if the predicate fails.
    ///
    /// # Panics
    /// - Panics if no event was emitted (i.e. `self.event()` is `None`).
    ///
    /// # Returns
    /// Returns `self`, allowing this method to be chained with other expectations.
    ///
    /// # Requirements
    /// The `E::Event` type must implement [`PartialEq`] and [`Debug`] for `assert_eq!`.
    pub fn assert_event(self, f: impl Fn(&E::Event)) -> Self {
        f(self.event());
        self
    }

    /// Returns a reference to the emitted event.
    ///
    /// This method retrieves the event that was emitted by a command execution,
    /// if one exists. It is typically used in tests to inspect the produced event.
    ///
    /// # Panics
    /// - Panics if no event was emitted (i.e., `self.event` is `None`).
    ///
    /// # Returns
    /// A reference to the emitted event.
    pub fn event(&self) -> &E::Event {
        match &self.event {
            Some(event) => event,
            _ => panic!("no events: error {:?}", self.error),
        }
    }

    /// Asserts that the command produced the expected reply.
    ///
    /// # Parameters
    /// - `expected_reply`: The value you expect the command to return.
    ///
    /// # Panics
    /// - Panics if no reply was produced.
    /// - Panics if the actual reply does not equal the expected reply.
    ///
    /// # Returns
    /// Returns `self` to allow method chaining.
    pub fn expect_reply(self, expected_reply: R) -> Self {
        let actual_reply = self.reply();
        assert_eq!(actual_reply, &expected_reply);
        self
    }

    /// Asserts (based on the provided function) that the command produced
    /// the expected reply.
    ///
    /// # Parameters
    /// - `f`: The function predicating the actual reply. The function must
    /// panic if the predicate fails.
    ///
    /// # Panics
    /// - Panics if no reply was produced.
    ///
    /// # Returns
    /// Returns `self` to allow method chaining.
    pub fn assert_reply(self, f: impl Fn(&R)) -> Self {
        f(self.reply());
        self
    }

    /// Returns a reference to the reply produced by the command.
    ///
    /// # Panics
    /// - Panics if no reply is present in the result.
    ///
    /// # Returns
    /// A reference to the reply value.
    pub fn reply(&self) -> &R {
        match &self.reply {
            Some(reply) => reply,
            _ => panic!("no reply"),
        }
    }

    /// Asserts that the command resulted in the expected error.
    ///
    /// # Panics
    /// - Panics if no error is present in the result.
    /// - Panics if the actual error does not equal the expected error.
    ///
    /// # Parameters
    /// - `expected_error`: The expected error value returned by the command.
    ///
    /// # Returns
    /// Returns `self` to allow method chaining.
    pub fn expect_error(self, expected_error: ER) -> Self {
        let actual_error = self.error();
        assert_eq!(actual_error, &expected_error);
        self
    }

    /// Asserts (using the supplied function) that the command resulted
    /// in the expected error.
    ///
    /// # Panics
    /// - Panics if no error is present in the result.
    ///
    /// # Parameters
    /// - `f`: The function predicating the actual error. The function must
    /// panic indicating predicate failure.
    ///
    /// # Returns
    /// Returns `self` to allow method chaining.
    pub fn assert_error(self, f: impl Fn(&ER)) -> Self {
        f(self.error());
        self
    }

    /// Returns a reference to the error produced by the command, if any.
    ///
    /// # Panics
    /// Panics if the command did not produce an error.
    ///
    /// # Returns
    /// A reference to the error.
    pub fn error(&self) -> &ER {
        match &self.error {
            Some(error) => error,
            _ => panic!("no error"),
        }
    }
}
