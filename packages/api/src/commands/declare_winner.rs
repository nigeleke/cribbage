use crate::{Error, Event, EventKind, Game, GameId, Player};
use eventsourced::*;

#[derive(Debug)]
pub struct DeclareWinner {
    game_id: GameId,
    winner: Player,
}

impl DeclareWinner {
    pub fn new(game_id: GameId, winner: Player) -> Self {
        Self { game_id, winner }
    }
}

impl Command<Game> for DeclareWinner {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let DeclareWinner { game_id: _, winner } = self;
        CommandEffect::emit(Event::new(*id, EventKind::WinnerDeclared { winner }))
    }
}
