use crate::domain::{Game, GameError, GameEvent, UserId};

pub struct HostGame {
    host: UserId,
    name: String,
}

impl HostGame {
    pub fn new(host: UserId, name: &str) -> Self {
        let name = String::from(name);
        Self { host, name }
    }
}

impl Command<Game, GameEvent, GameError> for HostGame {
    async fn execute(&self, _game: Game) -> Result<(Vec<Event>, Game), GameError> {
        let host = self.host;
        let name = self.name.clone();

        let game = Game::new_starting(host, None, &name);
        let events = Vec::from([Event::GameHosted { host, name }]);
        Ok((events, game))
    }
}
