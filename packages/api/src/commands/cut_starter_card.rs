use crate::{Discarding, Error, Event, EventKind, Game, GameId, State, prettify};
use eventsourced::{Command, CommandEffect};
use eventsourced_ext::lift_effect;

#[derive(Debug)]
pub struct CutStarterCard {
    game_id: GameId,
}

impl CutStarterCard {
    pub fn new(game_id: GameId) -> Self {
        Self { game_id }
    }
}

impl Command<Discarding> for CutStarterCard {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Discarding,
    ) -> CommandEffect<Discarding, Self::Reply, Self::Error> {
        let mut deck = state.deck().clone();
        let cut = deck.cut();
        CommandEffect::emit(Event::new(*id, EventKind::StarterCardCut { cut }))
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
                lift_effect!(
                    discarding,
                    CutStarterCard::new(*id).handle_command(id, discarding)
                )
            }
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
