use crate::{Error, Event, Game, GameId, Player, Starting, State, prettify};
use eventsourced::*;
use eventsourced_ext::lift_effect;

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

impl Command<Starting> for CutForDeal {
    type Reply = bool;
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Starting,
    ) -> CommandEffect<Starting, Self::Reply, Self::Error> {
        let player = self.player;

        let mut deck = state.deck().clone();
        let cut = deck.cut();

        let event = Event::CardCutForDeal {
            game_id: *id,
            player,
            cut,
        };

        CommandEffect::emit_and_reply(event, move |s: &Starting| {
            let mut pending = s.pending().clone();
            pending.acknowledge(player)
        })
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
            State::Starting(starting) => lift_effect!(
                starting,
                CutForDeal::new(*id, self.player).handle_command(id, starting)
            ),
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(CutForDeal))),
        }
    }
}
