use serde::{Deserialize, Serialize};

use super::Points;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pegging {
    back_peg: Points,
    front_peg: Points,
}

impl Pegging {
    #[cfg(test)]
    pub fn new(points: usize) -> Self {
        Self {
            back_peg: Points::from(0),
            front_peg: Points::from(points),
        }
    }

    pub const fn back_peg(&self) -> Points {
        self.back_peg
    }

    pub const fn front_peg(&self) -> Points {
        self.front_peg
    }

    pub fn points(&self) -> Points {
        self.front_peg
    }
}

impl std::ops::Add<Points> for Pegging {
    type Output = Self;

    fn add(mut self, rhs: Points) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Points> for Pegging {
    fn add_assign(&mut self, rhs: Points) {
        self.back_peg = self.front_peg;
        self.front_peg += rhs;
    }
}

impl std::fmt::Display for Pegging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            back_peg,
            front_peg,
        } = self;
        write!(f, "{back_peg:>3} -> {front_peg:>3}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scoring_will_jump_back_peg_over_front() {
        let mut score = Pegging {
            back_peg: Points::from(21),
            front_peg: Points::from(42),
        };
        let points = Points::from(15);

        score += points;

        assert_eq!(score.back_peg(), Points::from(42));
        assert_eq!(score.front_peg(), Points::from(57));
        assert_eq!(score.points(), Points::from(57));
    }

    #[test]
    fn score_can_be_displayed() {
        let score = Pegging {
            back_peg: Points::from(21),
            front_peg: Points::from(42),
        };

        insta::assert_snapshot!(score.to_string(), @" 21 ->  42");
    }
}
