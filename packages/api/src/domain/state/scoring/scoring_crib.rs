use super::common::Scoring;
use crate::{Event, GameId};
use eventsourced::EventSourced;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringCribType;
pub type ScoringCrib = Scoring<ScoringCribType>;

impl EventSourced for ScoringCrib {
    type Id = GameId;
    type Event = Event;

    const TYPE_NAME: &'static str = stringify!(ScoringCrib);

    fn handle_event(self, _event: Self::Event) -> Self {
        todo!()
    }
}
