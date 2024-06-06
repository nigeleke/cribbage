#[cfg(test)]
mod builder;

mod card;
mod cards;
mod constants;
mod format;
mod game;
mod player;
mod plays;
mod result;
mod score;
mod game_scorer;

pub mod prelude {
  pub use super::constants::*;

  #[cfg(test)]
  pub use crate::domain::builder::Builder;
  pub use crate::domain::*;
  pub use crate::domain::card::{Cut, Card, Rank, Value};
  pub use crate::domain::cards::{Crib, Cuts, Deck, Hand};
  pub use crate::domain::game::Game;
  pub use crate::domain::player::Player;
  pub use crate::domain::plays::*;
  pub use crate::domain::score::Score;
  pub use crate::domain::result::Error;
}
