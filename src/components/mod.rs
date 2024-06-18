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

pub(crate) mod prelude {
    pub use super::Context;
    pub use super::error::Error;
    pub use super::game::Game;
    pub use super::loading::Loading;
}

use crate::view::prelude::Game as GameView;

use leptos::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) id: String,
    pub(crate) state: RwSignal<Option<GameView>>,
}

impl Context {
    pub fn new(id: String, state: RwSignal<Option<GameView>>) -> Self {
        Self { id, state }
    }
}
