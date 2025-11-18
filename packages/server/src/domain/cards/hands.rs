use crate::domain::constants::*;
use crate::domain::{Hand, Player};

pub type Hands = [Hand; PLAYER_COUNT];

pub trait HasHands {
    fn hands(&self) -> &Hands;
    fn hands_mut(&mut self) -> &mut Hands;

    fn hand(&self, player: Player) -> &Hand {
        &self.hands()[player]
    }
}
