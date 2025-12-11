pub use crate::domain::{Card, Player};

/// The two cards cut by the players to determine who deals first.
///
/// Indices are host and guest respectively.
/// `None` means that player has not yet cut.
///
/// In Cribbage, the lower card wins the deal. Ties cause a redraw.
pub type CutsForDeal = [Option<Card>; 2];

/// Trait for game states that track the "cut for deal" phase.
pub trait HasCutsForDeal {
    /// Immutable access to both cuts.
    fn cuts_for_deal(&self) -> &CutsForDeal;

    /// Mutable access to both cuts.
    fn cuts_for_deal_mut(&mut self) -> &mut CutsForDeal;

    /// Returns the card cut by the given player, if any.
    #[must_use]
    fn cut_for_deal(&self, player: Player) -> Option<&Card> {
        self.cuts_for_deal()[player].as_ref()
    }

    /// Returns a mutable reference to the cut slot for the given player.
    #[must_use]
    fn cut_for_deal_mut(&mut self, player: Player) -> &mut Option<Card> {
        &mut self.cuts_for_deal_mut()[player]
    }
}
