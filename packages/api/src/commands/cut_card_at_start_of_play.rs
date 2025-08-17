use crate::{Discarding, Error, Event, Game, GameId, State, prettify};
use eventsourced::{Command, CommandEffect};
use eventsourced_ext::lift_effect;

#[derive(Debug)]
pub struct CutCardAtStartOfPlay {
    game_id: GameId,
}

impl CutCardAtStartOfPlay {
    pub fn new(game_id: GameId) -> Self {
        Self { game_id }
    }
}

impl Command<Discarding> for CutCardAtStartOfPlay {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        state: &Discarding,
    ) -> CommandEffect<Discarding, Self::Reply, Self::Error> {
        let mut deck = state.deck().clone();
        let cut = deck.cut();
        let event = Event::CardCutAtStartOfPlay { game_id: *id, cut };
        CommandEffect::emit(event)
    }
}

impl Command<Game> for CutCardAtStartOfPlay {
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
                    CutCardAtStartOfPlay::new(*id).handle_command(id, discarding)
                )
            }
            _ => CommandEffect::reject(Self::Error::NotPermitted(prettify!(CutCardAtStartOfPlay))),
        }
    }
}
