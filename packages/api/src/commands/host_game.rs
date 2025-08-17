use crate::{
    Event, Game, GameId, Player, UserId, domain::PLAYER0, name_builder::generate_game_name,
};
use eventsourced::{Command, CommandEffect, EventSourced};

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
    type Error = std::convert::Infallible;

    fn handle_command(
        self,
        _id: &GameId,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let game_id = GameId::new();
        let host = self.host;
        let name = generate_game_name();

        let event = Event::LobbyGameCreated {
            game_id,
            host,
            name,
        };

        CommandEffect::emit_and_reply(event, move |_| (game_id, PLAYER0))
    }
}
