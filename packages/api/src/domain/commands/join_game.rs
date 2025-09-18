use eventsourced::{Command, CommandEffect};

use crate::domain::PLAYER1;
use crate::{GameError, Event, Game, GameId, Player, UserId, prettify};

#[derive(Debug)]
pub struct JoinGame {
    game_id: GameId,
    guest: UserId,
}

impl JoinGame {
    pub fn new(game_id: GameId, guest: UserId) -> Self {
        Self { game_id, guest }
    }
}

impl Command<Game> for JoinGame {
    type Reply = Player;
    type Error = GameError;

    fn handle_command(
        self,
        id: &GameId,
        game: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        if game.id() != id {
            return CommandEffect::reject(GameError::InvalidGame(*id));
        };

        if game.guest().is_some() {
            CommandEffect::reject(GameError::NotPermitted(prettify!(JoinGame)))
        } else if game.host() == &self.guest {
            CommandEffect::reject(GameError::InvalidOpponent(self.guest))
        } else {
            let guest = self.guest;
            CommandEffect::emit_and_reply(Event::lobby_game_joined(*id, guest), move |_| PLAYER1)
        }
    }
}
