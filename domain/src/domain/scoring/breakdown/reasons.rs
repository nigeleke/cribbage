use super::reason::{Reason, ReasonType};
use crate::domain::{Card, Points};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Reasons(Vec<Reason>);

impl Reasons {
    pub fn with_fifteen(&mut self, cards: &[Card], points: Points) {
        self.0.push(Reason::new(ReasonType::Fifteen, cards, points));
    }

    pub fn with_pairs(&mut self, cards: &[Card], points: Points) {
        self.0.push(Reason::new(ReasonType::Pair, cards, points));
    }

    pub fn with_run(&mut self, cards: &[Card], points: Points) {
        self.0.push(Reason::new(ReasonType::Run, cards, points));
    }

    pub fn with_flush(&mut self, cards: &[Card], points: Points) {
        self.0.push(Reason::new(ReasonType::Flush, cards, points));
    }

    pub fn with_his_heels(&mut self, cards: &[Card], points: Points) {
        self.0
            .push(Reason::new(ReasonType::HisHeels, cards, points));
    }

    pub fn with_end_of_play(&mut self, cards: &[Card], points: Points) {
        self.0
            .push(Reason::new(ReasonType::EndOfPlay, cards, points));
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add(&mut self, reasons: &[Reason]) {
        let mut reasons = Vec::from(reasons);
        self.0.append(&mut reasons);
    }

    pub fn points(&self) -> Points {
        self.0.iter().map(|i| i.points()).sum()
    }
}

impl std::ops::Add for Reasons {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs = self.0;
        let mut rhs = rhs.0;
        lhs.append(&mut rhs);
        Self(lhs)
    }
}

impl std::ops::AddAssign for Reasons {
    fn add_assign(&mut self, rhs: Self) {
        let mut rhs = rhs.0;
        self.0.append(&mut rhs);
    }
}

impl std::fmt::Display for Reasons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reasons = self.0.iter().map(|c| c.to_string()).collect::<Vec<_>>();
        write!(f, "[{}]", reasons.join(", "))
    }
}
