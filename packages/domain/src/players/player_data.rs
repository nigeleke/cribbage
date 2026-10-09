use crate::constants::PLAYERS_PER_GAME;
use crate::players::Player;

/// A pair of values, one for each player, indexable by `Player`.
#[derive(Clone, PartialEq, Eq)]
pub struct PlayerData<T>([T; PLAYERS_PER_GAME]);

impl<T> PlayerData<T> {
    /// Returns an iterator over each T belonging to the players.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> std::default::Default for PlayerData<T>
where
    T: std::default::Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> From<[T; PLAYERS_PER_GAME]> for PlayerData<T> {
    fn from(value: [T; 2]) -> Self {
        Self(value)
    }
}

impl<T> std::ops::Index<Player> for PlayerData<T> {
    type Output = T;

    fn index(&self, player: Player) -> &Self::Output {
        &self.0[player.index()]
    }
}

impl<T> std::ops::IndexMut<Player> for PlayerData<T> {
    fn index_mut(&mut self, player: Player) -> &mut Self::Output {
        &mut self.0[player.index()]
    }
}

impl<T> std::fmt::Debug for PlayerData<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}
