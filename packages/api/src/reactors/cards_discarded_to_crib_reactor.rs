use crate::{CutCardAtStartOfPlay, Event, Game, GameId, State, prettify};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;

pub struct CardsDiscardedToCribReactor;

impl Reactor<Game> for CardsDiscardedToCribReactor {
    fn apply(&self, mut context: Game, id: &GameId, event: Event) -> Game {
        if let Event::CardsDiscardedToCrib {
            game_id,
            player: _,
            discards: _,
        } = event
        {
            let State::Discarding(discarding) = &context.state() else {
                panic!(
                    "unexpected state: {}",
                    prettify!(CardsDiscardedToCribReactor)
                );
            };

            let proceed = discarding.pending().finished();

            if proceed {
                let effect = CutCardAtStartOfPlay::new(game_id).handle_command(id, &context);
                if let CommandEffect::EmitAndReply(event, _) = effect {
                    context = context.handle_event(event);
                };
            };
        }

        context
    }
}
