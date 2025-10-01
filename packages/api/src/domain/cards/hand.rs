use super::pile::Pile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandType;
pub type Hand = Pile<HandType>;
