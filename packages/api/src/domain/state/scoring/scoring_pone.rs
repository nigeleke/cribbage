use super::common::Scoring;
use crate::{Event, GameId};
use eventsourced::EventSourced;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringPoneType;
pub type ScoringPone = Scoring<ScoringPoneType>;

impl EventSourced for ScoringPone {
    type Id = GameId;
    type Event = Event;

    const TYPE_NAME: &'static str = stringify!(ScoringPone);

    fn handle_event(self, _event: Self::Event) -> Self {
        todo!()
    }
}
