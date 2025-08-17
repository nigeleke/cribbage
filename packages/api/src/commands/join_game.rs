use crate::{Error, Event, Game, GameId, Player, Starting, UserId, domain::PLAYER1, prettify};
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
        state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        assert_eq!(id, state.id());

        if state.guest().is_some() {
            CommandEffect::reject(Error::NotPermitted(prettify!(JoinGame)))
        } else if state.host() == &self.guest {
            CommandEffect::reject(Error::InvalidOpponent(self.guest))
        } else {
            let guest = self.guest;

            let event = Event::LobbyGameJoined {
                game_id: *id,
                guest,
            };

            CommandEffect::emit_and_reply(event, move |_| PLAYER1)
        }
    }
}
