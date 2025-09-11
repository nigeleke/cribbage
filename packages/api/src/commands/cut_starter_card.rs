use crate::{Error, Event, EventKind, Game, GameId, State, prettify};
use eventsourced::{Command, CommandEffect};

#[derive(Debug)]
pub struct CutStarterCard {
    game_id: GameId,
}

impl CutStarterCard {
    pub fn new(game_id: GameId) -> Self {
        Self { game_id }
    }
}

impl Command<Game> for CutStarterCard {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Discarding(discarding) => {
                let mut deck = discarding.deck().clone();
                let cut = deck.cut();
                CommandEffect::emit(Event::new(*id, EventKind::StarterCardCut { cut }))
            }
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
