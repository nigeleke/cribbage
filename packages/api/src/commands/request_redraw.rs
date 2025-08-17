use crate::{Event, Game, GameId};
use eventsourced::{Command, CommandEffect};
use std::convert::Infallible;

#[derive(Debug)]
pub struct RequestRedraw {
    game_id: GameId,
}

impl RequestRedraw {
    pub fn new(game_id: GameId) -> Self {
        Self { game_id }
    }
}

impl Command<Game> for RequestRedraw {
    type Reply = ();

    type Error = Infallible;

    fn handle_command(
        self,
        id: &GameId,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        CommandEffect::emit(Event::RedrawRequested { game_id: *id })
    }
}
