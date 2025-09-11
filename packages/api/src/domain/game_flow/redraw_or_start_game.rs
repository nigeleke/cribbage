use crate::{Dealer, Event, EventKind, Game, GameId, RequestRedraw, Scoreboard, StartGame, State};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;
use std::cmp::{Ord, Ordering};

pub struct RedrawOrStartGame;

impl Reactor<Game> for RedrawOrStartGame {
    fn apply(&self, mut context: Game, id: &GameId, event: Event) -> Game {
        match &(event.kind(), context.state()) {
            (EventKind::CardCutForDeal { player, cut }, State::Starting(starting)) => {
                let cuts = starting.cuts();
                let pending = starting.pending();

                let proceed = pending.finished();
                if proceed {
                    let opponent = player.opponent();
                    let opponent_cut = cuts[opponent];
                    let dealer = match cut.value().cmp(&opponent_cut.value()) {
                        Ordering::Less => Some(Dealer::from(*player)),
                        Ordering::Greater => Some(Dealer::from(opponent)),
                        Ordering::Equal => None,
                    };

                    let effect = if let Some(dealer) = dealer {
                        let scoreboard = Scoreboard::default();
                        StartGame::new(*id, dealer, scoreboard).handle_command(id, &context)
                    } else {
                        RequestRedraw::new(*id).handle_command(id, &context)
                    };

                    if let CommandEffect::EmitAndReply(event, _) = effect {
                        context = context.handle_event(event);
                    };
                }
            }
            _ => {}
        }

        context
    }
}
