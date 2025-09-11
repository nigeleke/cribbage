use crate::{CutStarterCard, Event, EventKind, Game, GameId, State};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;

pub struct CutStarterCardAfterDiscards;

impl Reactor<Game> for CutStarterCardAfterDiscards {
    fn apply(&self, mut context: Game, id: &GameId, event: Event) -> Game {
        if let EventKind::CardsDiscardedToCrib {
            player: _,
            discards: _,
        } = event.kind()
            && let State::Discarding(discarding) = &context.state()
        {
            let proceed = discarding.pending().finished();

            if proceed {
                let effect = CutStarterCard::new(*id).handle_command(id, &context);
                if let CommandEffect::EmitAndReply(event, _) = effect {
                    context = context.handle_event(event);
                };
            };
        }

        context
    }
}
