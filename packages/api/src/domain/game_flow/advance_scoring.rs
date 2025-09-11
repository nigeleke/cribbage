use crate::{AcknowledgePoneScore, DeclareWinner, Event, EventKind, Game, GameId, State, prettify};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;

pub struct AdvanceScoring;

impl Reactor<Game> for AdvanceScoring {
    fn apply(&self, mut context: Game, id: &GameId, event: Event) -> Game {
        match (event.kind(), context.state()) {
            (EventKind::PoneHandScored { breakdown }, State::ScoringPone(scoring)) => {}
            (EventKind::PoneHandScoreAcknowledged { player }, State::ScoringPone(scoring)) => {}
            (EventKind::DealerHandScored { breakdown }, State::ScoringDealer(scoring)) => {}
            (EventKind::DealerHandScoreAcknowledged { player }, State::ScoringDealer(scoring)) => {}
            (EventKind::CribScored { breakdown }, State::ScoringCrib(scoring)) => {}
            (EventKind::CribScoreAcknowledged { player }, State::ScoringCrib(scoring)) => {}
            _ => {}
        }

        context
    }
}
