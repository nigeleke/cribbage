use crate::{
    Event, EventKind, Game, GameId, Player, UserId, domain::PLAYER0,
    name_builder::generate_game_name,
};
use eventsourced::{Command, CommandEffect, EventSourced};

#[derive(Debug)]
pub struct PlayComputer {
    host: UserId,
}

impl PlayComputer {
    pub fn new(host: UserId) -> Self {
        Self { host }
    }
}

impl Command<Game> for PlayComputer {
    type Reply = (GameId, Player);

    type Error = std::convert::Infallible;

    fn handle_command(
        self,
        _id: &<Game as EventSourced>::Id,
        _state: &Game,
    ) -> CommandEffect<Game, Self::Reply, Self::Error> {
        let game_id = GameId::new();
        let host = self.host;
        let computer = UserId::default();
        let users = [host, computer];
        let name = generate_game_name();

        CommandEffect::emit_and_reply(
            Event::new(game_id, EventKind::ComputerGameStarted { users, name }),
            move |_| (game_id, PLAYER0),
        )
    }
}
