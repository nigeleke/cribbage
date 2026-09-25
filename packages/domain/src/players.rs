mod dealer;
mod player;
mod pone;
mod roles;

pub use dealer::Dealer;
pub use player::{PLAYERS, Player};
pub use pone::Pone;
pub use roles::Roles;

// ------------------------------------
/// A pair of values, one for each player, indexable by `Player`.
#[derive(Clone, PartialEq, Eq)]
pub struct Players<T>([T; 2]);

impl<T> Players<T> {
    /// Returns an iterator over each T belonging to the players.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> std::default::Default for Players<T>
where
    T: std::default::Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> From<[T; 2]> for Players<T> {
    fn from(value: [T; 2]) -> Self {
        Self(value)
    }
}

impl<T> std::ops::Index<Player> for Players<T> {
    type Output = T;

    fn index(&self, player: Player) -> &Self::Output {
        &self.0[player.index()]
    }
}

impl<T> std::ops::IndexMut<Player> for Players<T> {
    fn index_mut(&mut self, player: Player) -> &mut Self::Output {
        &mut self.0[player.index()]
    }
}

impl<T> std::fmt::Debug for Players<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}
