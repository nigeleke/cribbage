use crate::{Error, Event, Game, GameId, Player, UserId, domain::PLAYER1, prettify};
use eventsourced::{Command, CommandEffect};

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
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        game: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        if game.id() != id {
            return CommandEffect::reject(Error::InvalidGame(*id));
        };

        if game.guest().is_some() {
            CommandEffect::reject(Error::NotPermitted(prettify!(JoinGame)))
        } else if game.host() == &self.guest {
            CommandEffect::reject(Error::InvalidOpponent(self.guest))
        } else {
            let guest = self.guest;
            CommandEffect::emit_and_reply(Event::lobby_game_joined(*id, guest), move |_| PLAYER1)
        }
    }
}
