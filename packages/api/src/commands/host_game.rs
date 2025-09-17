use crate::{
    Error, Event, Game, GameId, Player, UserId, domain::PLAYER0, name_builder::generate_game_name,
};
use eventsourced::{Command, CommandEffect};

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
    type Error = Error;

    fn handle_command(
        self,
        id: &GameId,
        game: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        if game.id() != &GameId::default() {
            return CommandEffect::reject(Error::InvalidGame(*id));
        };

        let game_id = *id;
        let host = self.host;
        let name = generate_game_name();

        CommandEffect::emit_and_reply(Event::lobby_game_created(game_id, host, name), move |_| {
            (game_id, PLAYER0)
        })
    }
}
