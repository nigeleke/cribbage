use crate::cqrs::Command;
use crate::domain::{DomainError, Event, Game, UserId};

pub struct JoinGame {
    guest: UserId,
}

impl JoinGame {
    pub fn new(guest: UserId) -> Self {
        Self { guest }
    }
}

impl Command<Game, Event, DomainError> for JoinGame {
    async fn execute(&self, mut game: Game) -> Result<(Vec<Event>, Game), DomainError> {
        let guest = self.guest;

        if game.guest().is_some() {
            Err(DomainError::NotPermitted(String::from("join game")))
        } else if game.host() == &guest {
            Err(DomainError::InvalidOpponent(guest))
        } else {
            game.add_guest(guest);
            let events = Vec::from([Event::GameJoined { guest }]);
            Ok((events, game))
        }
    }
}
