use crate::{
    Event, EventKind, Game, GameId, Player, UserId, domain::PLAYER0,
    name_builder::generate_game_name,
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
    type Error = std::convert::Infallible;

    fn handle_command(
        self,
        _id: &GameId,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let game_id = GameId::new();
        let host = self.host;
        let name = generate_game_name();

        CommandEffect::emit_and_reply(
            Event::new(game_id, EventKind::LobbyGameCreated { host, name }),
            move |_| (game_id, PLAYER0),
        )
    }
}
