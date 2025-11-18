pub use crate::domain::{Card, Player};

pub type CutsForDeal = [Option<Card>; 2];

pub trait HasCutsForDeal {
    fn cuts_for_deal(&self) -> &CutsForDeal;
    fn cuts_for_deal_mut(&mut self) -> &mut CutsForDeal;

    fn cut_for_deal(&self, player: Player) -> Option<&Card> {
        self.cuts_for_deal()[player].as_ref()
    }

    fn cut_for_deal_mut(&mut self, player: Player) -> &mut Option<Card> {
        &mut self.cuts_for_deal_mut()[player]
    }
}
