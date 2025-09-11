use crate::{Event, EventKind, Game, GameId, State};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;

pub struct CheckForEndOfPlays;

impl Reactor<Game> for CheckForEndOfPlays {
    fn apply(&self, mut context: Game, id: &GameId, event: Event) -> Game {
        let end_of_play = match (context.state(), event.kind()) {
            (State::Playing(playing), EventKind::CardPlayed { player, card }) => {
                playing.play_state().all_cards_are_played()
            }
            (State::Playing(playing), EventKind::Passed { player }) => {
                playing.play_state().all_cards_are_played()
            }
            _ => false,
        };

        if end_of_play {
            let event = Event::new(*id, EventKind::PlaysFinished);
            context = context.handle_event(event);
        }

        context
    }
}
