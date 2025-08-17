use super::pile::Pile;

/// The current Crib.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CribType;
pub type Crib = Pile<CribType>;
