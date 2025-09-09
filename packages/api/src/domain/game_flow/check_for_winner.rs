use crate::{DeclareWinner, Event, Game, GameId, State};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;

pub struct CheckForWinner;

impl Reactor<Game> for CheckForWinner {
    fn apply(&self, mut context: Game, id: &GameId, _event: Event) -> Game {
        if let Some(scoreboard) = match context.state() {
            State::Playing(playing) => Some(playing.scoreboard()),
            State::ScoringPone(scoring) => Some(scoring.scoreboard()),
            State::ScoringDealer(scoring) => Some(scoring.scoreboard()),
            State::ScoringCrib(scoring) => Some(scoring.scoreboard()),
            _ => None,
        } {
            if let Some(winner) = scoreboard.winner() {
                let effect = DeclareWinner::new(*id, winner).handle_command(id, &context);
                if let CommandEffect::EmitAndReply(event, _) = effect {
                    context = context.handle_event(event);
                }
            }
        }

        context
    }
}
