use crate::{Dealer, Event, EventKind, Game, GameId, Scoreboard};
use eventsourced::{Command, CommandEffect};
use std::convert::Infallible;

#[derive(Debug)]
pub struct StartGame {
    game_id: GameId,
    dealer: Dealer,
    scoreboard: Scoreboard,
}

impl StartGame {
    pub fn new(game_id: GameId, dealer: Dealer, scoreboard: Scoreboard) -> Self {
        Self {
            game_id,
            dealer,
            scoreboard,
        }
    }
}

impl Command<Game> for StartGame {
    type Reply = ();
    type Error = Infallible;

    fn handle_command(
        self,
        id: &GameId,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let dealer = self.dealer;
        let scoreboard = self.scoreboard;

        CommandEffect::emit(Event::new(
            *id,
            EventKind::RoundStarted { dealer, scoreboard },
        ))
    }
}
