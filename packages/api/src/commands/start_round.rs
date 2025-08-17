use crate::{Dealer, Event, Game, GameId, Scoreboard};
use eventsourced::{Command, CommandEffect};
use std::convert::Infallible;

#[derive(Debug)]
pub struct StartRound {
    game_id: GameId,
    dealer: Dealer,
    scoreboard: Scoreboard,
}

impl StartRound {
    pub fn new(game_id: GameId, dealer: Dealer, scoreboard: Scoreboard) -> Self {
        Self {
            game_id,
            dealer,
            scoreboard,
        }
    }
}

impl Command<Game> for StartRound {
    type Reply = ();
    type Error = Infallible;

    fn handle_command(
        self,
        _id: &GameId,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let StartRound {
            game_id,
            dealer,
            scoreboard,
        } = self;
        let event = Event::RoundStarted {
            game_id,
            dealer,
            scoreboard,
        };
        CommandEffect::emit(event)
    }
}
