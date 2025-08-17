use crate::{Error, Event, Game, GameId, Player, Playing, State, prettify};
use eventsourced::*;
use eventsourced_ext::lift_effect;

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
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        match state.state() {
            State::Playing(playing) => lift_effect!(
                playing,
                DeclareWinner::new(*id, self.winner).handle_command(id, playing)
            ),
            _ => CommandEffect::reject(Error::NotPermitted(prettify!(DeclareWinner))),
        }
    }
}

impl Command<Playing> for DeclareWinner {
    type Reply = ();
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        _state: &Playing,
    ) -> CommandEffect<Playing, Self::Reply, Self::Error> {
        let DeclareWinner { game_id: _, winner } = self;
        let event = Event::WinnerDeclared {
            game_id: *id,
            winner,
        };
        CommandEffect::emit(event)
    }
}
