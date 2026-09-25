use crate::Points;

/// Represents a player's two peg positions on the scoreboard.
///
/// `front` is the player's current score and `back` is the position
/// previously occupied by the front peg.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    back: Points,
    front: Points,
}

impl Position {
    /// Returns the points in the back position.
    pub const fn back(self) -> Points {
        self.back
    }

    /// Returns the points in the front position.
    pub const fn front(self) -> Points {
        self.front
    }

    /// Returns the total points for the position (same as `front`).
    pub const fn points(self) -> Points {
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

#[cfg(test)]
mod tests {
    use crate::Points;

    use super::*;

    #[test]
    fn default_position_has_zero_points() {
        let position = Position::default();
        assert_eq!(position.back(), Points::default());
        assert_eq!(position.front(), Points::default());
        assert_eq!(position.points(), Points::default());
    }

    #[test]
    fn position_points_are_current_front_peg() {
        let position = Position {
            back: Points::new(10),
            front: Points::new(15),
        };

        assert_eq!(position.points(), Points::new(15));
    }

    #[test]
    fn adding_points_advances_front_peg_and_moves_previous_front_to_back() {
        let position = Position {
            back: Points::new(10),
            front: Points::new(15),
        };

        let position = position + Points::new(4);

        assert_eq!(position.back(), Points::new(15));
        assert_eq!(position.front(), Points::new(19));
    }

    #[test]
    fn adding_assign_advances_front_peg_and_moves_previous_front_to_back() {
        let mut position = Position {
            back: Points::new(10),
            front: Points::new(15),
        };

        position += Points::new(4);

        assert_eq!(position.back(), Points::new(15));
        assert_eq!(position.front(), Points::new(19));
    }

    #[test]
    fn successive_scoring_advances_both_pegs() {
        let mut position = Position::default();

        position += Points::new(10);

        assert_eq!(position.back(), Points::new(0));
        assert_eq!(position.front(), Points::new(10));

        position += Points::new(5);

        assert_eq!(position.back(), Points::new(10));
        assert_eq!(position.front(), Points::new(15));
    }

    #[test]
    fn adding_zero_still_moves_front_peg_to_back() {
        let position = Position {
            back: Points::new(10),
            front: Points::new(15),
        };

        let position = position + Points::default();

        assert_eq!(position.back(), Points::new(15));
        assert_eq!(position.front(), Points::new(15));
    }
}
