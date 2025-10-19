use eventsourced::{Command, CommandEffect};

use crate::domain::{DomainError, Event, Game, GameId, PLAYER1, Player, UserId};
use crate::prettify;

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
    type Error = DomainError;

    fn handle_command(
        self,
        id: &GameId,
        game: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        if game.id() != id {
            return CommandEffect::reject(DomainError::InvalidGame(*id));
        };

        if game.guest().is_some() {
            CommandEffect::reject(DomainError::NotPermitted(prettify!(JoinGame)))
        } else if game.host() == &self.guest {
            CommandEffect::reject(DomainError::InvalidOpponent(self.guest))
        } else {
            let guest = self.guest;
            CommandEffect::emit_and_reply(Event::lobby_game_joined(*id, guest), move |_| PLAYER1)
        }
    }
}
