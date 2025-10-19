use super::pile::Pile;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CribType;
pub type Crib = Pile<CribType>;
