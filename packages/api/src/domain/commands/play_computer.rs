use eventsourced::{Command, CommandEffect};

use crate::domain::PLAYER0;
use crate::name_builder::generate_game_name;
use crate::{GameError, Event, Game, GameId, Player, UserId};

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
        let guest = UserId::default();
        let name = generate_game_name();

        CommandEffect::emit_and_reply(
            Event::computer_game_created(game_id, host, guest, name),
            move |_| (game_id, PLAYER0),
        )
    }
}
