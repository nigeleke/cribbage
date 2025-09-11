use crate::{Error, Event, EventKind, Game, GameId, Player, State, prettify};
use eventsourced::*;

#[derive(Debug)]
pub struct CutForDeal {
    game_id: GameId,
    player: Player,
}

impl CutForDeal {
    pub fn new(game_id: GameId, player: Player) -> Self {
        Self { game_id, player }
    }
}

impl Command<Game> for CutForDeal {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Starting(starting) => {
                let player = self.player;

                let mut deck = starting.deck().clone();
                let cut = deck.cut();

                let mut pending = starting.pending().clone();
                let proceed = pending.acknowledge(player);

                CommandEffect::emit_and_reply(
                    Event::new(*id, EventKind::CardCutForDeal { player, cut }),
                    move |_| proceed,
                )
            }
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(CutForDeal))),
        }
    }
}
