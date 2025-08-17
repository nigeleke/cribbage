use eventsourced::EventSourced;

/// A `Reactor` observes events and produces follow-up commands based on the new state of the entity.
///
/// `E` - The entity triggering the Reactor.
pub trait Reactor<E>
where
    E: EventSourced,
{
    /// Apply the events.
    fn apply(&self, context: E, id: &E::Id, event: E::Event) -> E;
}
