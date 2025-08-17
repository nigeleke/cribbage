use super::pile::Pile;

/// A player's hand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandType;
pub type Hand = Pile<HandType>;
