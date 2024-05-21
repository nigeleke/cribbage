mod card;
mod cards;
mod crib;
mod cuts;
mod error;
mod game;
mod loading;

pub mod prelude {
    pub use super::Context;
    pub use super::error::Error;
    pub use super::game::Game;
    pub use super::loading::Loading;
}

use leptos::*;

use crate::view::Game as GameView;

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
