use eventsourced::{Command, CommandEffect};

use crate::domain::PLAYER0;
use crate::name_builder::generate_game_name;
use crate::{GameError, Event, Game, GameId, Player, UserId};

#[derive(Debug)]
pub struct HostGame {
    host: UserId,
}

impl HostGame {
    pub fn new(host: UserId) -> Self {
        Self { host }
    }
}

impl Command<Game> for HostGame {
    type Reply = (GameId, Player);
    type Error = GameError;

    fn handle_command(
        self,
        id: &GameId,
        game: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        if game.id() != &GameId::default() {
            return CommandEffect::reject(GameError::InvalidGame(*id));
        };

        let game_id = *id;
        let host = self.host;
        let name = generate_game_name();

        CommandEffect::emit_and_reply(Event::lobby_game_created(game_id, host, name), move |_| {
            (game_id, PLAYER0)
        })
    }
}
