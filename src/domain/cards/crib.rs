use super::cards::Cards;

/// The current Crib.
#[derive(Clone, Debug, PartialEq)]
pub struct CribType;
pub type Crib = Cards<CribType>;
