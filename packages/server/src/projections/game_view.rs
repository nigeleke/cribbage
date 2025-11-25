use crate::domain::Game;
use cqrs_es::{EventEnvelope, View};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GameView;

impl View<Game> for GameView {
    fn update(&mut self, _event: &EventEnvelope<Game>) {
        todo!()
    }
}
