mod card;
mod cards;
mod crib;
mod cutting_for_start;
mod discarding;
mod error;
mod game;
mod loading;
mod playing;
mod plays;
mod scoreboard;
mod scoring;

pub use self::error::Error;
pub use self::game::Game;
pub use self::loading::Loading;

use crate::view::Game as GameView;

use leptos::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub id: String,
    pub state: RwSignal<Option<GameView>>,
}

impl Context {
    pub fn new(id: String, state: RwSignal<Option<GameView>>) -> Self {
        Self { id, state }
    }
}
