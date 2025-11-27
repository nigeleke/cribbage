use cqrs_es::{EventEnvelope, View};
use serde::{Deserialize, Serialize};

use crate::domain::Game;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameView {
    instance: Game,
}

impl GameView {
    pub fn instance(&self) -> &Game {
        &self.instance
    }
}

impl View<Game> for GameView {
    fn update(&mut self, event: &EventEnvelope<Game>) {
        self.instance.apply_event(event.payload.clone());
    }
}
