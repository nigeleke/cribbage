use crate::{
    Dealer, Event, Game, GameId, HasFace, HasValue, RequestRedraw, Scoreboard, StartRound, State,
    prettify,
};
use eventsourced::{Command, CommandEffect, EventSourced};
use eventsourced_ext::Reactor;
use std::cmp::{Ord, Ordering};

pub struct CardCutForDealReactor;

impl Reactor<Game> for CardCutForDealReactor {
    fn apply(&self, mut context: Game, id: &GameId, event: Event) -> Game {
        if let Event::CardCutForDeal {
            game_id,
            player,
            cut,
        } = event
        {
            let State::Starting(starting) = context.state().clone() else {
                panic!("unexpected state: {}", prettify!(CardCutForDealReactor));
            };

            let (cuts, _deck, pending_status) = starting.into_parts();

            let proceed = pending_status.finished();

            if proceed {
                let opponent = player.opponent();
                let opponent_cut = cuts[opponent];
                let dealer = match cut.face().value().cmp(&opponent_cut.face().value()) {
                    Ordering::Less => Some(Dealer::from(player)),
                    Ordering::Greater => Some(Dealer::from(opponent)),
                    Ordering::Equal => None,
                };

                let effect = if let Some(dealer) = dealer {
                    let scoreboard = Scoreboard::default();
                    StartRound::new(game_id, dealer, scoreboard).handle_command(id, &context)
                } else {
                    RequestRedraw::new(game_id).handle_command(id, &context)
                };

                if let CommandEffect::EmitAndReply(event, _) = effect {
                    context = context.handle_event(event);
                };
            };
        }

        context
    }
}
