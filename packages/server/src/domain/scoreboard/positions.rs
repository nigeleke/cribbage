use crate::domain::Position;

/// Represents the positions of both players in the game.
///
/// The array contains two `Position` elements:
/// - Index `0` corresponds to `PLAYER0`
/// - Index `1` corresponds to `PLAYER1`
pub type Positions = [Position; 2];
