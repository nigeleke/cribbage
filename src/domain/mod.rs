#[cfg(test)]
mod builder;

mod card;
mod cards;
mod constants;
mod game;
mod player;
mod plays;
mod result;
mod score;

pub mod prelude {
  pub use super::constants::*;

  #[cfg(test)]
  pub use crate::domain::builder::Builder;
  pub use crate::domain::*;
  pub use crate::domain::card::{Card, Rank, Value};
  pub use crate::domain::cards::{Crib, Deck, Hand};
  pub use crate::domain::game::Game;
  pub use crate::domain::player::Player;
  pub use crate::domain::plays::*;
  pub use crate::domain::score::Score;
  pub use crate::domain::result::Error;
}
