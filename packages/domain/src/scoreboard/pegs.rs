use crate::Points;

/// One player's pair of pegs on the board.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Pegs {
    /// Current score (front peg).
    front: Points,
    /// Previous score (back peg).
    back: Points,
}

impl Pegs {
    /// Record a score: the back peg hops over the front peg.
    /// Returns the new total.
    pub fn score(&mut self, points: Points) -> Points {
        self.back = self.front;
        self.front += points;
        self.front
    }

    /// Return the current front peg points representation.
    pub fn front_peg(self) -> Points {
        self.front
    }

    /// Return the current back peg points representation.
    pub fn back_peg(self) -> Points {
        self.back
    }
}

impl std::fmt::Debug for Pegs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}->{:?}", self.back, self.front)
    }
}
