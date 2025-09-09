use super::common::Scoring;
use crate::{Event, GameId};
use eventsourced::EventSourced;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringDealerType;
pub type ScoringDealer = Scoring<ScoringDealerType>;

impl EventSourced for ScoringDealer {
    type Id = GameId;
    type Event = Event;

    const TYPE_NAME: &'static str = stringify!(ScoringDealer);

    fn handle_event(self, _event: Self::Event) -> Self {
        todo!()
    }
}
