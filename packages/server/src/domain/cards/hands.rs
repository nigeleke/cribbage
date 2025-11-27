use crate::domain::{Hand, Player, constants::*};

pub type Hands = [Hand; PLAYER_COUNT];

pub trait HasHands {
    fn hands(&self) -> &Hands;
    fn hands_mut(&mut self) -> &mut Hands;

    fn hand(&self, player: Player) -> &Hand {
        &self.hands()[player]
    }

    fn hand_mut(&mut self, player: Player) -> &mut Hand {
        &mut self.hands_mut()[player]
    }
}
