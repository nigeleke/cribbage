use crate::{DeclareWinner, Event, Game, GameId, State};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;

pub struct ScoringReactor;

impl Reactor<Game> for ScoringReactor {
    fn apply(&self, mut context: Game, id: &GameId, event: Event) -> Game {
        println!("ScoringReactor<Game> {event:?} {}", context.state());

        if let Some(scoreboard) = match context.state() {
            State::Playing(playing) => Some(playing.scoreboard()),
            _ => None,
        } {
            if let Some(winner) = scoreboard.winner() {
                println!("Scoring Reactor - declaring winner");
                let effect = DeclareWinner::new(*id, winner).handle_command(id, &context);
                if let CommandEffect::EmitAndReply(event, _) = effect {
                    context = context.handle_event(event);
                }
            }
        }

        context
    }
}
