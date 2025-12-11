use serde::{Deserialize, Serialize};

use super::Points;

/// Represents a player's position in the game.
///
/// Tracks points in the "front" and "back" peg positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    back: Points,
    front: Points,
}

impl Position {
    #[cfg(test)]
    pub(crate) fn new(points: usize) -> Self {
        Self {
            back: Points::from(0),
            front: Points::from(points),
        }
    }

    /// Returns the points in the back position.
    #[must_use]
    pub const fn back(&self) -> Points {
        self.back
    }

    /// Returns the points in the front position.
    #[must_use]
    pub const fn front(&self) -> Points {
        self.front
    }

    /// Returns the total points for the position (same as `front`).
    #[must_use]
    pub fn points(&self) -> Points {
        self.front
    }
}

impl std::ops::Add<Points> for Position {
    type Output = Self;

    fn add(mut self, rhs: Points) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Points> for Position {
    fn add_assign(&mut self, rhs: Points) {
        self.back = self.front;
        self.front += rhs;
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            back: back_peg,
            front: front_peg,
        } = self;
        write!(f, "{back_peg:>3} -> {front_peg:>3}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scoring_will_jump_back_peg_over_front() {
        let mut score = Position {
            back: Points::from(21),
            front: Points::from(42),
        };
        let points = Points::from(15);

        score += points;

        assert_eq!(score.back(), Points::from(42));
        assert_eq!(score.front(), Points::from(57));
        assert_eq!(score.points(), Points::from(57));
    }

    #[test]
    fn score_can_be_displayed() {
        let score = Position {
            back: Points::from(21),
            front: Points::from(42),
        };

        insta::assert_snapshot!(score.to_string(), @" 21 ->  42");
    }
}
